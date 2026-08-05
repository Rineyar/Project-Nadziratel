use std::time::Instant;

use image::{DynamicImage, ImageReader, GrayImage, RgbImage}; //Для считывания
use image::imageops::{resize, FilterType::Lanczos3}; //Для увеличения
use imageproc::contrast::{otsu_level, threshold, ThresholdType::Binary}; //Для бинаризации
use image::imageops::invert; //Для инверсии
use imageproc::{distance_transform::Norm::LInf, morphology::{erode, dilate}}; //Для чистки

use oar_ocr::prelude::*; //Для распознования

/*
//Когда потребуется читать с байтов
fn read_img_from_bytes() -> DynamicImage
{
    use std::fs::read;
    use image::{load_from_memory_with_format, guess_format};

    let raw_img: Vec<u8> = read("img.png").expect("Read IMG error!\n");

    return load_from_memory_with_format(&raw_img, guess_format(&raw_img).expect("Unknown format!\n")).expect("Load IMG error!\n");
}
*/

fn gray_to_rgb(img: &GrayImage) -> RgbImage
{
    return DynamicImage::ImageLuma8(img.clone()).into_rgb8();
}

fn main() 
{
    let time_start: Instant = Instant::now();

    let img: DynamicImage = ImageReader::open("img.png").expect("Open IMG error!\n").decode().expect("Decode error!\n");

    let gray: GrayImage = img.to_luma8();
    gray.save("gray.png").unwrap();

    let resized: GrayImage = resize(&gray, gray.width() * 2, gray.height() * 2, Lanczos3);
    resized.save("resized.png").unwrap();

    let binary: GrayImage = threshold(&resized, otsu_level(&resized), Binary);
    binary.save("binary.png").unwrap();

    let mut inverted: GrayImage = binary.clone();
    invert(&mut inverted);
    let inverted: GrayImage = inverted;
    inverted.save("inverted.png").unwrap();

    let dilated_inv: GrayImage = dilate(&inverted, LInf, 1);
    dilated_inv.save("dilated_inv.png").unwrap();

    let eroded_inv: GrayImage = erode(&inverted, LInf, 1);
    eroded_inv.save("eroded_inv.png").unwrap();

    let dilated: GrayImage = dilate(&binary, LInf, 1);
    dilated.save("dilated.png").unwrap();

    let eroded: GrayImage = erode(&binary, LInf, 1);
    eroded.save("eroded.png").unwrap();

    let ocr: OAROCR = OAROCRBuilder::new(
    "models/pp-ocrv5_mobile_det.onnx", //Ищет текст
    "models/eslav_pp-ocrv5_mobile_rec.onnx", //Пытается понять че написано
    "models/ppocrv5_eslav_dict.txt") //Словарь
    .build().expect("OCR build error!\n");

    let res: Vec<OAROCRResult> = ocr.predict(vec![gray_to_rgb(&gray)]).expect("OCR error!\n");

    for region in &res[0].text_regions
    {
        if let Some((text, confidence)) = region.text_with_confidence()
        {
            println!("{text} — {confidence:.2}");
        }
    }

    println!("{:?}", time_start.elapsed());
}
