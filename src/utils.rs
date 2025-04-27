use std::{fs::{self, File}, io::{BufReader, Cursor, Read}};

use datetime::{Month, Weekday};
use image::{DynamicImage, ImageReader};
use serde_json::Value;



pub fn list_folder_configs(folder: &str) -> Vec<String> {
    let mut ret: Vec<String> = vec![];
    let dir = fs::read_dir(folder).unwrap();
    for dir_entry in dir {
        let path_buf = dir_entry.unwrap().path();
        let path = path_buf.as_os_str().to_str().unwrap();
        if path.ends_with(".json") {
            let s_pos = path.rfind("/").unwrap_or(0) + 1;
            let e_pos = path.len() - 5;
            let name = &path[s_pos..e_pos];
            ret.push(name.to_string());
        }
    };
    ret
}

pub fn load_config(folder: &str, name: &str) -> Value {
    let path = format!("{}/{}.json", folder, name);
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let config: Value = serde_json::from_reader(reader).unwrap();
    config
}

pub fn get_image(uri: &str) -> Option<DynamicImage> {
    if uri.starts_with("http://") || uri.starts_with("https://") {
        let mut res = reqwest::blocking::get(uri).unwrap();
        let mut image_bytes: Vec<u8> = vec![];
        let _ = res.read_to_end(&mut image_bytes);
        let image = ImageReader::new(Cursor::new(image_bytes)).with_guessed_format().unwrap().decode().unwrap();
        return Some(image);
    }
    None
}

pub fn get_month_name(m: Month) -> String {


    (match m {
        Month::January =>"January",
        Month::February => "February",
        Month::March => "March",
        Month::April => "April",
        Month::May => "May",
        Month::June => "June",
        Month::July => "July",
        Month::August => "August",
        Month::September => "September",
        Month::October => "October",
        Month::November => "November",
        Month::December => "December",
    }).to_string()
}

pub fn get_weekday_name(wd: Weekday) -> String {
    (match wd {
        Weekday::Sunday => "Sunday",
        Weekday::Monday => "Monday",
        Weekday::Tuesday => "Tuesday",
        Weekday::Wednesday => "Wednesday",
        Weekday::Thursday => "Thursday",
        Weekday::Friday => "Friday",
        Weekday::Saturday => "Saturday",
    }).to_string()
}