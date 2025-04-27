use std::rc::Rc;

use image::{DynamicImage, ImageReader};
use serde_json::Value;

use crate::{data::DataStore, layout::{imagebox::ImageBox, LayoutItem}};

use super::Component;


pub struct ImageUnit {
    image: Rc<DynamicImage>
}

impl ImageUnit {
    pub fn new(value: &Value) -> Self {
        let path = value["file"].as_str().unwrap();
        let image = ImageReader::open(path).unwrap().decode().unwrap();
        ImageUnit{ image: Rc::new(image) }
    }
}

impl Component for ImageUnit {
    fn produce(&self, _data_store: &DataStore) -> Box<dyn LayoutItem> {
        let image_box = ImageBox::new(self.image.clone());
        Box::new(image_box)
    }
}