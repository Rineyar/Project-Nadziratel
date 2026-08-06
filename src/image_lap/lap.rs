use image::{DynamicImage, ImageError, ImageFormat, RgbImage, guess_format, load_from_memory_with_format}; //База
// use image::{DynamicImage, ImageReader, GrayImage}; //Для считывания
// use image::imageops::{resize, FilterType::Lanczos3}; //Для увеличения
// use imageproc::contrast::{otsu_level, threshold, ThresholdType::Binary}; //Для бинаризации
// use image::imageops::invert; //Для инверсии
// use imageproc::{distance_transform::Norm::LInf, morphology::{erode, dilate}}; //Для чистки

// pub fn read_img_from_bytes(path: &str) -> RgbImage
// {
//     use std::fs::read;

//     let raw_img: Vec<u8> = read(path).expect("Read IMG error!\n");

//     return load_from_memory_with_format(&raw_img, guess_format(&raw_img).expect("Unknown format!\n")).expect("Load IMG error!\n").into_rgb8();
// }

pub fn img_from_bytes(bytes: &[u8]) -> Result<RgbImage, ImageError>
{
    let form: ImageFormat = guess_format(bytes)?;

    let img: DynamicImage = load_from_memory_with_format(bytes, form)?;

    return Ok(img.into_rgb8());
}

// pub fn gray_to_rgb(img: GrayImage) -> RgbImage
// {
//     return DynamicImage::ImageLuma8(img).into_rgb8();
// }

// pub fn direct_read(path: &str) -> RgbImage
// {
//     return ImageReader::open(path).expect("Open IMG error!\n").decode().expect("Decode error!\n").into_rgb8();
// }

// pub fn get_resized_rbg(img: &RgbImage) -> RgbImage
// {
//     return resize(img, img.width() / 2, img.height() / 2, Lanczos3);
// }

// pub fn get_variables(img: &RgbImage) -> (RgbImage, GrayImage, GrayImage, GrayImage, GrayImage, GrayImage, GrayImage, GrayImage)
// {
//     let resized: RgbImage = resize(img, img.width() * 2, img.height() * 2, Lanczos3);

//     let gray: GrayImage = DynamicImage::ImageRgb8(resized.clone()).into_luma8();

//     let binary: GrayImage = threshold(&gray, otsu_level(&gray), Binary);

//     let mut inverted: GrayImage = binary.clone();
//     invert(&mut inverted);
//     let inverted: GrayImage = inverted;

//     let dilated_inv: GrayImage = dilate(&inverted, LInf, 1);

//     let eroded_inv: GrayImage = erode(&inverted, LInf, 1);

//     let dilated: GrayImage = dilate(&binary, LInf, 1);

//     let eroded: GrayImage = erode(&binary, LInf, 1);

//     return (resized, gray, binary, inverted, dilated_inv, eroded_inv, dilated, eroded);
// }