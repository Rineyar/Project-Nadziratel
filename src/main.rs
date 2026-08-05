use std::fs::read;
use image::{load_from_memory_with_format, guess_format, DynamicImage};

fn main() 
{
    let raw_img: Vec<u8> = read("test.png").expect("Read IMG error!\n"); //Чтение изображения

    println!("Raw len: {:?}", raw_img.len());

    //Получить чистое изображение
    let img: DynamicImage = load_from_memory_with_format(&raw_img, guess_format(&raw_img).expect("Unknown format!\n")).expect("Load IMG error!\n");

    println!("Clear len: {:?}", img.into_bytes().len());
}
