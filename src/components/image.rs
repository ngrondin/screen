use std::rc::Rc;

use image::ImageReader;
use serde_json::Value;

use crate::{data::DataStore, layout::{imagebox::ImageBox, LayoutItem}};

use super::Component;


pub struct ImageUnit {
    path: String
    //image: Rc<DynamicImage>
}

impl ImageUnit {
    pub fn new(value: &Value) -> Self {
        let path = value["file"].as_str().unwrap();
        
        ImageUnit{ path: path.to_string() }
    }
}

impl Component for ImageUnit {
    fn produce(&self, _data_store: &DataStore) -> Box<dyn LayoutItem> {
        let image = ImageReader::open(&self.path).unwrap().decode().unwrap();
        let image_box = ImageBox::new(Rc::new(image));
        Box::new(image_box)
    }
}