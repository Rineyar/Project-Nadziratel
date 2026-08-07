use std::fs::read_to_string; //Считать настройки и токен
use serenity::all::{Client, GatewayIntents, ChannelId, Http}; //Бот
use serenity::all::ShardManager;
use tokio::sync::mpsc::{channel}; //Связь
use oar_ocr::prelude::OAROCR; //Тип распознователя
use image::RgbImage; //Тип изображения
use std::{thread, thread::JoinHandle}; //Для распознования
use std::sync::Arc; //Для завершителя и передачек
use std::collections::HashMap; //Для словаря

//Для логов
use std::time::{Instant, Duration};
use chrono::{Local, prelude::DateTime};
use tracing_appender::{rolling::{never, RollingFileAppender}, non_blocking};
use tracing::{error, info, warn};
use serde::Serialize;
use std::fs::{File, OpenOptions, write};
use std::io::Write;

//Крейт
mod discord; //Обработчик событий
use discord::{Handler, MsgData};

mod image_lap; //Загрузка изображения из байт
use image_lap::img_from_bytes;

mod ocr; //Поиск текста и чистка
use ocr::{build_ocr, get_clear_text};

mod text_analisation; //Подсчёт слов
use text_analisation::{AnalysisResult, score_text, load_dict};

const SCORE_LIMIT: usize = 6;

#[derive(Serialize)]
struct ImgLog
{
    message_id: u64,
    channel_id: u64,
    user_id: u64,
    username: String,
    time: String,
    message_text: String,
    attachment_id: u64,
    filename: String,
    ocr_text: String,
    score: usize,
    matches: Vec<String>,
    raw_path: String,
    prog_path: String,
    log_path: String
}

#[tokio::main]
async fn main()
{
    let time_start: Instant = Instant::now();

    println!("Попытка запуститься...");

    let start_date: DateTime<Local> = Local::now();

    let log_file: String = format!("run_{}.log", start_date.format("%Y-%m-%d_%H-%M-%S"));

    let file_appender: RollingFileAppender = never("logs/program", log_file);

    let (loger, _guard) = non_blocking(file_appender);

    tracing_subscriber::fmt().with_writer(loger).with_ansi(false).init();

    info!("Loger started: {:?}", time_start.elapsed());
    println!("Логер запустился...");

    let token: String = read_to_string("env/token.local").expect("Token read error!\n").trim().to_string(); //Получить токен и остальное ниже

    info!("Bot token readed: {:?}", time_start.elapsed());
    println!("Токен считался...");

    let adm_id: u64 = read_to_string("env/admin.local").expect("Admin id read error!\n").trim().parse::<u64>().expect("Admin id parse error!\n");

    info!("Admin id readed: {:?}", time_start.elapsed());
    println!("ID админа считалось...");

    //Области доступа
    let intents: GatewayIntents =
        GatewayIntents::GUILDS //События и сервер
        | GatewayIntents::GUILD_MESSAGES //Новые сообщения
        | GatewayIntents::MESSAGE_CONTENT; //Содержание сообщений

    let (img_tx, mut img_rx) = channel::<Option<(Vec<u8>, MsgData)>>(32); //Канал связи с картинками

    let (call_tx, mut call_rx) = channel::<(u8, ChannelId)>(32); //Канал связи с обраткой

    let handler: Handler = Handler { img_tx: img_tx.clone() }; //Обработчик

    //Определить тело и передать обработчик
    let mut client: Client = Client::builder(token, intents).event_handler(handler).await.expect("Bot build error!\n");

    //Отправитель сообщений
    let http: Arc<Http> = client.http.clone();

    info!("Bot builder ready: {:?}", time_start.elapsed());
    println!("Бот почти готов к старту...");

    //Синхронный поток обработки картинок
    let thread: JoinHandle<(usize, Duration)> = thread::spawn(move ||
    {
        //Счётчик работы
        let (mut count, mut time_spended): (usize, Duration) = (0, Duration::ZERO);

        let ocr: OAROCR = build_ocr(); //Распознователь

        info!("OCR builded: {:?}", time_start.elapsed());
        println!("Распознователь запустился...");

        let dict: HashMap<String, usize> = load_dict("env/words.txt"); //Словарь

        info!("Dict loaded: {:?}", time_start.elapsed());
        println!("Словарь загрузился...");

        let mut json_file: File = OpenOptions::new().create(true).append(true).open(format!("logs/log_{}.jsonl", 
        start_date.format("%Y-%m-%d_%H-%M-%S"))).expect("JSON log open error!\n"); //Файл с логами о картинках в текущую сессию

        info!("Json opened: {:?}", time_start.elapsed());
        println!("Файл с логами открылся...");

        while let Some(message) = img_rx.blocking_recv() //Приёмка
        {
            let Some((bytes, data)) = message else //Распаковка
            {
                break;
            };

            let time_to_work: Instant = Instant::now();

            let channel_id: ChannelId = data.channel_id;

            warn!("Taked image: {} B", bytes.len());
            println!("Получено изображение: {} байт. Пу пу пу...", bytes.len());

            let img: RgbImage = match img_from_bytes(&bytes) //Загрузка картинки с байт
            {
                Ok(ok) => ok,

                Err(err) =>
                {
                    error!("Img load error!\n{:?}", err);
                    println!("Ошибка загрузки изображения! См. логи...");
                    continue;
                }
            };

            match write(format!("logs/images/raw_{}_{}_{}.png", data.user_id, data.message_id, data.att_id), bytes)
            {
                Ok(()) => {}

                Err(err) =>
                {
                    error!("Raw img save error!\n{:?}", err);
                    println!("Ошибка сохранения изображения! См. логи...");
                }
            }

            match img.save(format!("logs/images/prog_{}_{}_{}.png", data.user_id, data.message_id, data.att_id))
            {
                Ok(()) => {}

                Err(err) =>
                {
                    error!("Prog img save error!\n{:?}", err);
                    println!("Ошибка сохранения изображения! См. логи...");
                }
            }

            let text: Vec<String> = match get_clear_text(&ocr, vec![img]) //Получение текста с картинки
            {
                Ok(ok) => ok,

                Err(err) =>
                {
                    error!("OCR error!\n{:?}", err);
                    println!("Ошибка распознования текса на картинке! См. логи...");
                    continue;     
                }
            };

            let mut matches: Vec<String> = Vec::with_capacity(4);
            let mut score: usize = 0;
            let mut full_text: String = String::with_capacity(256);

            for (i, elem) in text.iter().enumerate() //Каждой картинке свой текст
            {
                let res: AnalysisResult = score_text(elem, &dict); //Результат

                warn!("N{} Score: {}", i, res.score);
                println!("У изображения: {} результат подозрительности. Пу пу пу...", res.score);
                
                if res.score > SCORE_LIMIT //Что-то есть
                {
                    match call_tx.blocking_send((1, channel_id)) //Знак в обратку, что нечисто
                    {
                        Ok(()) => 
                        {
                            warn!("ADMIN call sended, matches:");
                            println!("Админ вызван (возможно)...");
                        }

                        Err(err) =>
                        {
                            error!("Call send error!\n{:?}", err);
                            println!("Ошибка вызова админа! См. логи...");
                        }
                    }

                    println!("Подозрительное:");

                    for match_ in res.matches.iter()
                    {
                        warn!("{}", match_);
                        println!("{}", match_);

                        matches.push(match_.to_string());
                    }

                    println!("Как-то так...");
                } /*else {
                    match call_tx.blocking_send((0, ch_id)) //Знак в обратку, что чисто
                    {
                        Ok(()) => {}

                        Err(err) =>
                        {
                            error!("Call send error!\n{:?}", err);
                        }
                    }
                }*/
                
                score += res.score;
                
                full_text.push_str(elem);
            }

            let record: ImgLog = ImgLog
            {
                message_id: data.message_id.get(),
                channel_id: data.channel_id.get(),
                user_id: data.user_id.get(),
                username: data.username,
                time: data.time.to_string(),
                message_text: data.text,
                attachment_id: data.att_id.get(),
                filename: data.filename,
                ocr_text: full_text,
                score: score,
                matches: matches,
                raw_path: format!("logs/images/raw_{}_{}_{}.png", data.user_id, data.message_id, data.att_id),
                prog_path: format!("logs/images/prog_{}_{}_{}.png", data.user_id, data.message_id, data.att_id),
                log_path: format!("logs/program/run_{}.log", start_date.format("%Y-%m-%d_%H-%M-%S"))
            };

            match serde_json::to_string(&record)
            {
                Ok(json) =>
                {
                    if let Err(err) = writeln!(json_file, "{}", json)
                    {
                        error!("JSON write error!\n{:?}", err);
                        println!("Ошибка записи логов! См. логи...");
                    }
                }

                Err(err) =>
                {
                    error!("JSON serialize error!\n{:?}", err);
                    println!("Ошибка... Преобразования огромной структуры в json строку. См. логи...");
                }
            }

            //+1 к сделанным
            count += 1;
            time_spended += time_to_work.elapsed();
        }

        return (count, time_spended); //Поскольку поток move, нужно делать возврат
    });

    info!("Sync thread started: {:?}", time_start.elapsed());
    println!("Поток с обработчиком запустился...");

    //Поток на отправку сообщений
    tokio::spawn(async move
    {
        while let Some((res, channel_id)) = call_rx.recv().await //Ждём сообщение
        {
            if res == 0
            {
                continue;
            } else if res == 1
            {
                //Отправить сообщение в дискорд
                match channel_id.say(&http,format!("<@{adm_id}> обнаружено подозрительное изображение")).await
                {
                    Ok(_) => 
                    {
                        warn!("Admin called");
                        println!("Админ вызван...");
                    }

                    Err(err) =>
                    {
                        error!("Message send error!\n{:?}", err);
                        println!("Ошибка отправки сообщения! См. логи...");
                    }
                }
            }
        }
    });

    info!("Msg async thread started: {:?}", time_start.elapsed());
    println!("Поток для отправки сообщений запустился...");

    //Ссылка для завершителя
    let shard_manager: Arc<ShardManager> = client.shard_manager.clone();

    //Поток на завершение
    tokio::spawn(async move
    {
        //Ctrl + C завершает
        tokio::signal::ctrl_c().await.expect("Ctrl+C handler error!\n");

        info!("Завершение");
        println!("Завершение...");

        shard_manager.shutdown_all().await; //Закрытие бота

        info!("Bot stopped async: {:?}", time_start.elapsed());
        println!("Процесс с ботом остановлен...");
    });

    info!("Ender async thread started: {:?}", time_start.elapsed());
    println!("Поток для контроля Ctrl + C запустился...");
    
    info!("Bot started: {:?}", time_start.elapsed());
    println!("Бот запустился (вероятнее всего)...");

    //Старт бота
    client.start().await.expect("Bot start error!\n");

    match img_tx.send(None).await //Отправка в sync поток для остановки
    {
        Ok(()) => {}

        Err(err) => 
        {
            error!("None send error!\n{:?}", err);
            println!("Поток для отправки сообщений запустился...");      
        } 
    };

    //Чистка
    drop(client);
    drop(img_tx);

    info!("Bot stopped sync: {:?}", time_start.elapsed());
    println!("Бот остановился...");

    match thread.join() //Сбор sync потока
    {
        Ok((count, time_spended)) => 
        {
            if count == 0
            {
                info!("No work in this run");
                println!("Работы небыло...");
            } else {
                info!("Avg time to work: {:.4}s", time_spended.as_secs_f64() / count as f64);
                println!("Среднее время работы: {:.4}с...", time_spended.as_secs_f64() / count as f64);
            }

            info!("Thread joined: {:?}", time_start.elapsed());
            println!("Поток обработчика остановлен...");
        }

        Err(err) =>
        {
            error!("Thread join error!\n{:?}", err);
            println!("Ошибка остановки потока обработчика! См. логи...");
        }
    }

    info!("End: {:?}", time_start.elapsed());
}
