use std::{fs, rc::Rc};
use rand::Rng;

use image::ImageReader;
use serde_json::Value;

use crate::{data::DataStore, layout::{imagebox::ImageBox, LayoutItem}};

use super::Component;


pub struct RandomImageUnit {
    folder: String
}

impl RandomImageUnit {
    pub fn new(value: &Value) -> Self {
        let folder = value["folder"].as_str().unwrap();
        RandomImageUnit{ folder: folder.to_string() }
    }
}

impl Component for RandomImageUnit {
    fn produce(&self, _data_store: &DataStore) -> Box<dyn LayoutItem> {
        let dir = fs::read_dir(&self.folder).unwrap();
        let mut paths: Vec<String> = vec![];
        for dir_entry in dir {
            let path_buf = dir_entry.unwrap().path();
            let path = path_buf.as_os_str().to_str().unwrap();
            if path.ends_with(".jpg") || path.ends_with(".jpeg") || path.ends_with(".png") {
                paths.push(path.to_string());
            }
        };
        let mut rng = rand::rng();
        let i = rng.random_range(0..paths.len());
        let path = paths.get(i).unwrap();
        let image = ImageReader::open(path).unwrap().decode().unwrap();
        let image_box = ImageBox::new(Rc::new(image));
        Box::new(image_box)
    }
}