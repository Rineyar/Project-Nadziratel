use image::RgbImage;
use oar_ocr::prelude::*; //Для распознования

pub fn ocrresult_to_string(ocrresult: &OAROCRResult) -> String
{
    let mut result: String = String::with_capacity(1024);

    for region in &ocrresult.text_regions
    {
        if let Some((text, _confidence)) = region.text_with_confidence()
        {
            if !result.is_empty()
            {
                result.push(' ');
            }

            result.push_str(text);
        }
    }

    return result;
}

pub fn normalize(text: &str) -> String
{
    let mut result: String = String::with_capacity(text.len());
    let mut need_space: bool = false;

    for ch in text.chars().flat_map(char::to_lowercase)
    {
        let ch: char = match ch
        {
            'ё' => 'е',
            _ => ch,
        };

        if ch.is_alphanumeric()
        {
            if need_space && !result.is_empty()
            {
                result.push(' ');
            }

            result.push(ch);

            need_space = false;
        } else {
            need_space = true;
        }
    }

    return result;
}

pub fn build_ocr() -> OAROCR
{
    return OAROCRBuilder::new(
    "models/pp-ocrv5_mobile_det.onnx", //Ищет текст
    "models/eslav_pp-ocrv5_mobile_rec.onnx", //Пытается понять че написано
    "models/ppocrv5_eslav_dict.txt") //Словарь
    .build().expect("OCR build error!\n");
}

pub fn get_clear_text(ocr: &OAROCR, images: Vec<RgbImage>) -> Result<Vec<String>, OCRError>
{
    let pred: Vec<OAROCRResult> = ocr.predict(images)?;

    let mut res: Vec<String> = Vec::with_capacity(pred.len());

    for elem in &pred
    {
        let text: String = normalize(&ocrresult_to_string(elem));

        if text.is_empty()
        {
            continue;
        }

        res.push(text);
    }

    return Ok(res);
}