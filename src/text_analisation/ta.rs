use strsim::levenshtein; //Для сравнения с эталоном
use std::collections::HashMap; //Хеш мапа

use std::fs::read_to_string; //Счение строки
use crate::ocr::normalize; //Нормализация

pub const PURE_TEXT: &str = "РИА НОВОСТИ 12:56 30.07.2026 (обновлено 14:34 30.07.2026) Поделиться Комментарии Меллстрой открыл своё казино 
    и в честь открытия раздаёт 10000 рублей каждому новому пользователю. Бонус можно получить на drgn36.com. 
    После регистрации деньги поступают сразу на баланс. Играть или выводить - решайте сами.";

#[derive(Debug, Clone)]
pub struct AnalysisResult
{
    pub score: usize,
    pub matches: Vec<String>,
}

pub fn cer(expected: &str, actual: &str) -> f64
{
    let distance: usize = levenshtein(&expected, &actual);
    let length: usize = expected.chars().count().max(1);

    return distance as f64 / length as f64;
}

pub fn load_dict(path: &str) -> HashMap<String, usize>
{
    let content: String = read_to_string(path).expect("Dictionary load error!\n");

    let mut dict: HashMap<String, usize> = HashMap::new();

    for line in content.lines()
    {
        let line: &str = line.trim();

        if line.is_empty() || line.starts_with('#')
        {
            continue;
        }

        let Some((key, weight)) = line.split_once('\t') else
        {
            eprintln!("Invalid dictionary line: {line}");
            continue;
        };

        let Ok(weight) = weight.trim().parse::<usize>() else
        {
            eprintln!("Invalid weight: {line}");
            continue;
        };

        dict.insert(normalize(key), weight);
    }

    return dict;
}

pub fn score_text(text: &str, dict: &HashMap<String, usize>) -> AnalysisResult
{
    let mut res: AnalysisResult = AnalysisResult { score: 0, matches: Vec::with_capacity(2) };

    for word in text.split_whitespace()
    {
        if let Some(weight) = dict.get(word)
        {
            res.score += *weight;
            res.matches.push(word.to_string());
        }
    }

    return res;
}