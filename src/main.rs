use image::{DynamicImage, ImageReader, GrayImage};

/*
//Когда потребуется читать с байтов
fn read_img_from_bytes() -> DynamicImage
{
    use std::fs::read;
    use image::{load_from_memory_with_format, guess_format};

    let raw_img: Vec<u8> = read("test.png").expect("Read IMG error!\n");

    return load_from_memory_with_format(&raw_img, guess_format(&raw_img).expect("Unknown format!\n")).expect("Load IMG error!\n");
}
*/

fn main() 
{
    let img: DynamicImage = ImageReader::open("test.png").expect("Open IMG error!\n").decode().expect("Decode error!\n");

    let gray: GrayImage = img.to_luma8();
    gray.save("gray.png").unwrap();
}
