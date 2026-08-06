use std::fs::read_to_string; //Считать токен
use serenity::all::{Client, GatewayIntents}; //Бот

mod discord; //Обработчик событий
use discord::Handler;

#[tokio::main]
async fn main()
{
    let token: String = read_to_string("env.local").expect("Token read error!\n").trim().to_string(); //Получить токен

    //Области доступа
    let intents: GatewayIntents =
        GatewayIntents::GUILDS //События и сервер
        | GatewayIntents::GUILD_MESSAGES //Новые сообщения
        | GatewayIntents::MESSAGE_CONTENT; //Содержание сообщений

    //Определить тело | Обработчик внешний
    let mut client: Client = Client::builder(token, intents).event_handler(Handler).await.expect("Bot build error!\n");

    //Старт бота
    client.start().await.expect("Bot start error!\n");

}

/*
use std::{collections::HashMap, time::{Duration, Instant}};

mod image_lap;
use image::RgbImage;
use image_lap::read_img_from_bytes;

mod ocr;
use ocr::{get_clear_text, build_ocr};

mod text_analisation;
use text_analisation::load_dict;
use text_analisation::score_text;
use text_analisation::AnalysisResult;

const TESTS: usize = 7;

fn main() 
{
    let time_start: Instant = Instant::now();

    let mut imgvec: Vec<RgbImage> = Vec::with_capacity(TESTS);

    for i in 0..TESTS
    {
        imgvec.push(read_img_from_bytes(&format!("imgs/img{}.png", i)));
    }

    let time_iml: Duration = time_start.elapsed();

    let ocr: oar_ocr::prelude::OAROCR = build_ocr();

    let time_ocrl: Duration = time_start.elapsed();

    let dict: HashMap<String, usize> = load_dict("src/text_analisation/words.txt");

    let time_dl: Duration = time_start.elapsed();

    let text: Vec<String> = get_clear_text(&ocr, imgvec);

    let time_tclr: Duration = time_start.elapsed();

    for (i, elem) in text.iter().enumerate()
    {
        let res: AnalysisResult = score_text(elem, &dict);

        println!("{} img:           {}", i, res.score);
        
        if res.score > 5
        {
            println!("ADMIN called, matches:");

            for match_ in res.matches.iter()
            {
                println!("{}", match_)
            }
        }
    }

    let time_end: Duration = time_start.elapsed();

    println!("Imgs load:        {:?}", time_iml);
    println!("OCR build:        {:?}", time_ocrl - time_iml);
    println!("Dicr load:        {:?}", time_dl - time_ocrl);
    println!("Texts clear:      {:?}", time_tclr - time_dl);
    println!("Score calc:       {:?}", time_end - time_tclr);
    println!("Total time:       {:?}", time_start.elapsed());
}
*/