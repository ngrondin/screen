use std::rc::Rc;

use serde_json::Value;

use crate::{data::DataStore, fonts::FontFactory, framebuffer::Color, layout::{textbox::TextBox, LayoutItem}};

use super::Component;


pub struct TextUnit {
    text: String,
    font_factory: Rc<FontFactory>,
    font_size: f32,
    color: Color
}

impl TextUnit {
    pub fn new(value: &Value, font_factory: Rc<FontFactory>) -> Self {
        let text = value["text"].as_str().or(Some("")).unwrap();
        let font_size = match value["fontsize"].as_u64() {
            Some(fontsize_num) => fontsize_num as f32,
            None => 18.0
        };
        let color = match value["color"].as_str() {
            Some(color_str) => Color::from_string(color_str),
            None => Color::new(240, 240, 240)
        };
        TextUnit{ text: text.to_string(), font_factory, font_size, color }
    }
}

impl Component for TextUnit {
    fn produce(&self, _data_store: &DataStore) -> Box<dyn LayoutItem> {
        let font = self.font_factory.get_font("DejaVuSans", self.font_size).unwrap();
        let textbox = TextBox::new(&self.text, &font, &self.color);
        Box::new(textbox)
    }
}