use std::rc::Rc;

use serde_json::Value;

use crate::{data::DataStore, fonts::{Font, FontFactory}, framebuffer::Color, layout::{containerbox::{ContainerAlign, ContainerBox, ContainerDir}, imagebox::ImageBox, textbox::TextBox}, models::weather::WeatherData, utils::get_image};

use super::Component;


pub struct WeatherUnit {
    data_name: String,
    title_font: Font,
    general_font: Font,
    color: Color,
}

impl WeatherUnit {
    pub fn new(value: &Value, font_factory: Rc<FontFactory>) -> Self {
        let data_name = value["data"].as_str().or(Some("")).unwrap();
        let title_font = font_factory.get_font("DejaVuSans", 80.0).unwrap();
        let general_font = font_factory.get_font("DejaVuSans", 50.0).unwrap();
        let color = Color::new(240, 240, 240);
        WeatherUnit { 
            data_name: data_name.to_string(),
            title_font,
            general_font,
            color
        }
    }


}

impl Component for WeatherUnit {
    fn produce(&self, data_store: &DataStore) -> Box<dyn crate::layout::LayoutItem> {
        let mut top = ContainerBox::new(ContainerDir::Column, ContainerAlign::Start, 0, 0, None);
        let data: WeatherData = data_store.load(&self.data_name);
        let mut title_box = ContainerBox::new(ContainerDir::Row, ContainerAlign::Center, 0, 0, None);
        if let Some(icon) = get_image(&data.icon) {
            title_box.add_content(Box::new(ImageBox::new(Rc::new(icon))));
        }
        title_box.add_content(Box::new(TextBox::new(&data.title, &self.title_font, &self.color)));
        top.add_content(Box::new(title_box));

        let mut line_box1 = ContainerBox::new(ContainerDir::Row, ContainerAlign::Center, 0, 5, None);
        line_box1.add_content(Box::new(TextBox::new(&format!("{}°C  ", (data.temp - 273.0) as u32), &self.general_font, &self.color)));
        line_box1.add_content(Box::new(TextBox::new(&format!("{}kPa  ", data.pressure), &self.general_font, &self.color)));
        top.add_content(Box::new(line_box1));

        let mut line_box2 = ContainerBox::new(ContainerDir::Row, ContainerAlign::Center, 0, 5, None);
        line_box2.add_content(Box::new(TextBox::new(&format!("{} ", get_wind_dir(data.wind_dir)), &self.general_font, &self.color)));
        line_box2.add_content(Box::new(TextBox::new(&format!("{}km/h  ", data.wind_speed as u32), &self.general_font, &self.color)));
        top.add_content(Box::new(line_box2));

        Box::new(top)
    }
}

pub fn get_wind_dir(dir: u64) -> String {
    (if dir <= 22 || dir > 342 {
        "N"
    } else if dir > 22 && dir <= 337 {
        "NE"
    } else if dir > 67 && dir <= 112 {
        "E"
    } else if dir > 112 && dir <= 157 {
        "SE"
    } else if dir > 157 && dir <= 202 {
        "S"
    } else if dir > 202 && dir <= 247 {
        "SW"
    } else if dir > 247 && dir <= 292 {
        "W"
    } else if dir > 292 && dir <= 337 {
        "NW"
    } else {
        "-"
    }).to_string()
}