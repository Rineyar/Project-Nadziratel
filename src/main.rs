use std::{collections::{HashMap, HashSet}, time::{Duration, Instant}};

mod image_lap;
use image_lap::read_img_from_bytes;

mod ocr;
use ocr::{get_clear_text, build_ocr};







fn main() 
{
    let time_start: Instant = Instant::now();

    let text: String = get_clear_text(build_ocr(), vec![read_img_from_bytes()]);

    let dict: HashMap<&str, isize> = HashMap::new();

    

    let time_end: Duration = time_start.elapsed();

    println!("{time_end:?}");
}


