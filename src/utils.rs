use std::{fs::{self, File}, io::{BufReader, Cursor, Read}};
use std::hash::{DefaultHasher, Hash, Hasher};
use chrono::Weekday;
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
    let image_bytes = if uri.starts_with("http://") || uri.starts_with("https://") {
        let uri_hash = calculate_hash(&uri.to_string());
        let uri_str_hash = format!("{:x}", uri_hash);
        let cache_path = format!("cache/{}", uri_str_hash);
        if fs::exists(&cache_path).unwrap() {
            match fs::read(&cache_path) {
                Ok(res) => Some(res),
                Err(_) => None,
            }
        } else {
            match reqwest::blocking::get(uri) {
                Ok(mut res) => {
                    let mut bytes: Vec<u8> = vec![];
                    let _ = res.read_to_end(&mut bytes);
                    let _ = fs::write(&cache_path, &bytes);
                    Some(bytes)
                },
                Err(_) => None,
            }
        }
    } else if uri.starts_with("file://") {
        let path = &uri[7..];
        match fs::read(path) {
            Ok(res) => Some(res),
            Err(_) => None,
        }
    } else {
        None
    };

    match image_bytes {
        Some(bytes) => {
            let image = ImageReader::new(Cursor::new(bytes)).with_guessed_format().unwrap().decode().unwrap();
            Some(image)
        },
        None => {
            None
        }
    }
}

pub fn get_month_name(m: u32) -> String {
    (match m {
        1 =>"January",
        2 => "February",
        3 => "March",
        4 => "April",
        5 => "May",
        6 => "June",
        7 => "July",
        8 => "August",
        9 => "September",
        10 => "October",
        11 => "November",
        12 => "December",
        _ => "Unknown"
    }).to_string()
}

pub fn get_weekday_name(wd: Weekday) -> String {
    (match wd {
        Weekday::Sun => "Sunday",
        Weekday::Mon => "Monday",
        Weekday::Tue => "Tuesday",
        Weekday::Wed => "Wednesday",
        Weekday::Thu => "Thursday",
        Weekday::Fri => "Friday",
        Weekday::Sat => "Saturday",
    }).to_string()
}


fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}