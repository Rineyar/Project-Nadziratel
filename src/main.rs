use image::{DynamicImage, ImageReader, ImageBuffer, Luma};

fn main() 
{
    let img: DynamicImage = ImageReader::open("test.png").expect("Open IMG error!\n").decode().expect("Decode error!\n");

    let gray: ImageBuffer<Luma<u8>, Vec<u8>> = img.to_luma8();
    gray.save("gray.png").unwrap();
}
