use std::fs::read_to_string; //Считать настройки и токен
use serenity::all::{Client, GatewayIntents, ShardManager, ChannelId, Http}; //Бот
use tokio::sync::mpsc::{channel}; //Связь
use oar_ocr::prelude::OAROCR;
use image::RgbImage;
use std::thread; //Для распознования
use std::sync::Arc; //Для завершителя
use std::collections::HashMap; //Для словаря

mod discord; //Обработчик событий
use discord::Handler;

mod image_lap;
use image_lap::img_from_bytes;

mod ocr;
use ocr::{build_ocr, get_clear_text};

mod text_analisation;
use text_analisation::{AnalysisResult, score_text, load_dict};

fn load_settings() -> (String, usize)
{
    let content: String = read_to_string("settings").expect("Settings load error!\n");

    let mut res: (String, usize) = ("".to_string(), 0);

    for line in content.lines()
    {
        let line: &str = line.trim();

        if line.is_empty() || line.starts_with('#')
        {
            continue;
        }

        let Some((key, data)) = line.split_once('\t') else
        {
            eprintln!("Invalid dictionary line: {line}");
            continue;
        };

        if key == "PTT"
        {
            res.0 = data.to_string();
        } else if key == "ADM_ID"
        {
            res.1 = data.parse::<usize>().expect("Invalid ADM_ID");
        }
    }

    return res;
}

#[tokio::main]
async fn main()
{
    let (ptt, adm_id) = load_settings();

    let token: String = read_to_string(ptt).expect("Token read error!\n").trim().to_string(); //Получить токен

    //Области доступа
    let intents: GatewayIntents =
        GatewayIntents::GUILDS //События и сервер
        | GatewayIntents::GUILD_MESSAGES //Новые сообщения
        | GatewayIntents::MESSAGE_CONTENT; //Содержание сообщений

    let (img_tx, mut img_rx) = channel::<(Vec<u8>, ChannelId)>(128); //Канал связи с картинками

    let (call_tx, mut call_rx) = channel::<(u8, ChannelId)>(16); //Канал связи с обраткой

    let handler: Handler = Handler { img_tx }; //Обработчик

    //Определить тело и передать обработчик
    let mut client: Client = Client::builder(token, intents).event_handler(handler).await.expect("Bot build error!\n");

    //Отправитель сообщений
    let http: Arc<Http> = client.http.clone();

    //Синхронный поток обработки картинок
    thread::spawn(move ||
    {
        let ocr: OAROCR = build_ocr(); //Распознователь

        let dict: HashMap<String, usize> = load_dict("src/text_analisation/words.txt"); //Словарь

        while let Some((bytes, ch_id)) = img_rx.blocking_recv() //Приёмка
        {
            println!("Получено изображение: {} байт", bytes.len());

            let img: RgbImage = img_from_bytes(&bytes);

            let text: Vec<String> = get_clear_text(&ocr, vec![img]);

            for (i, elem) in text.iter().enumerate() //Каждой картинке свой текст
            {
                let res: AnalysisResult = score_text(elem, &dict); //Результат

                println!("{} img: {}", i, res.score);
                
                if res.score > 5 //Что-то есть
                {
                    println!("ADMIN called, matches:");

                    match call_tx.blocking_send((1, ch_id)) //Знак в обратку, что нечисто
                    {
                        Ok(()) => {}
                        Err(err) =>
                        {
                            eprintln!("Call send error!\n{:?}", err);
                        }
                    }

                    for match_ in res.matches.iter()
                    {
                        println!("{}", match_)
                    }
                } else {
                    match call_tx.blocking_send((0, ch_id)) //Знак в обратку, что чисто
                    {
                        Ok(()) => {}
                        Err(err) =>
                        {
                            eprintln!("Call send error!\n{:?}", err);
                        }
                    }
                }
            }
        }
    });

    tokio::spawn(async move
    {
        while let Some((res, ch_id)) = call_rx.recv().await
        {
            if res == 0
            {
                continue;
            } else if res == 1
            {
                ch_id.say(&http,format!("@{adm_id} обнаружено подозрительное изображение")).await.expect("ADMIN call error!\n");
            }
        }
    });

    //Управление отсюда
    let shard_manager: Arc<ShardManager> = client.shard_manager.clone();

    //Поток на завершение
    tokio::spawn(async move
    {
        tokio::signal::ctrl_c().await.expect("Ctrl+C handler error!\n");

        println!("Завершение");

        shard_manager.shutdown_all().await;
    });
    
    //Старт бота
    client.start().await.expect("Bot start error!\n");

}
