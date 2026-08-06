use image::RgbImage;
use oar_ocr::prelude::*; //Для распознования

pub const PURE_TEXT: &str = "РИА НОВОСТИ 12:56 30.07.2026 (обновлено 14:34 30.07.2026) Поделиться Комментарии Меллстрой открыл своё казино 
    и в честь открытия раздаёт 10000 рублей каждому новому пользователю. Бонус можно получить на drgn36.com. 
    После регистрации деньги поступают сразу на баланс. Играть или выводить - решайте сами.";

pub fn ocrresult_to_string(ocrresult: &OAROCRResult) -> String
{
    return ocrresult.text_regions.iter().filter_map(|region|{region.text_with_confidence()
    .map(|(text, _confidence)| text)}).collect::<Vec<&str>>().join(" ");
}

pub fn normalize(text: &str) -> String
{
    return text.to_lowercase().trim().split_whitespace().collect::<Vec<&str>>().join(" ");
}

pub fn cer(expected: &str, actual: &str) -> f64
{
    use strsim::levenshtein;

    let expected: String = normalize(expected);
    let actual: String = normalize(actual);

    let distance: usize = levenshtein(&expected, &actual);
    let length: usize = expected.chars().count().max(1);

    return distance as f64 / length as f64;
}

pub fn build_ocr() -> OAROCR
{
    return OAROCRBuilder::new(
    "models/pp-ocrv5_mobile_det.onnx", //Ищет текст
    "models/eslav_pp-ocrv5_mobile_rec.onnx", //Пытается понять че написано
    "models/ppocrv5_eslav_dict.txt") //Словарь
    .build().expect("OCR build error!\n");
}

pub fn get_clear_text(ocr: OAROCR, images: Vec<RgbImage>) -> String
{
    let pred: Vec<OAROCRResult> = ocr.predict(images).expect("OCR error!\n");

    let mut res: String = String::new();

    for elem in pred.iter()
    {
        res.push_str(&normalize(&ocrresult_to_string(elem)));
    }

    return res;
}