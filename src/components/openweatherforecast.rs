use std::rc::Rc;

use chrono::{Datelike, Local, TimeZone, Utc};
use serde_json::Value;

use crate::{data::DataStore, fonts::{Font, FontFactory}, framebuffer::Color, layout::{containerbox::{ContainerAlign, ContainerBox, ContainerDir, ContainerFixedSize, ContainerJustify}, imagebox::ImageBox, textbox::TextBox}, models::weather::WeatherForecastData, utils::{get_image, get_month_name}};

use super::{openweather::get_wind_dir, Component};


pub struct WeatherForecastUnit {
    data_name: String,
    date_font: Font,
    title_font: Font,
    text_font: Font,
    color: Color,
}

impl WeatherForecastUnit {
    pub fn new(value: &Value, font_factory: Rc<FontFactory>) -> Self {
        let data_name = value["data"].as_str().or(Some("")).unwrap();
        let date_font = font_factory.get_font("DejaVuSans", 35.0).unwrap();
        let title_font = font_factory.get_font("DejaVuSans", 40.0).unwrap();
        let text_font = font_factory.get_font("DejaVuSans", 30.0).unwrap();
        let color = Color::new(240, 240, 240);
        WeatherForecastUnit { 
            data_name: data_name.to_string(),
            date_font,
            title_font,
            text_font,
            color
        }
    }
}

impl Component for WeatherForecastUnit {
    fn produce(&self, data_store: &DataStore) -> Box<dyn crate::layout::LayoutItem> {
        let mut top = ContainerBox::new(ContainerDir::Column, ContainerAlign::Start, ContainerJustify::Start, 0, 0, None);
        let data: WeatherForecastData = data_store.load(&self.data_name);

        for item in data.list {
            let mut line_box = ContainerBox::new(ContainerDir::Row, ContainerAlign::Center, ContainerJustify::Start, 0, 5, None);

            let mut date_box = ContainerBox::new_fixed_size(ContainerDir::Row, ContainerAlign::Center, ContainerJustify::Start, ContainerFixedSize{width: 230, height:40}, 0, None);
            let date_time = Utc.timestamp_opt(item.ts, 0).unwrap().with_timezone(&Local);
            let date_str = format!("{} {}", date_time.day(), get_month_name(date_time.month()));
            date_box.add_content(Box::new(TextBox::new(&date_str, &self.date_font, &self.color)));
            line_box.add_content(Box::new(date_box));

            if let Some(icon) = get_image(&item.icon) {
                line_box.add_content(Box::new(ImageBox::new(Rc::new(icon))));
            }

            let mut line_text_col = ContainerBox::new(ContainerDir::Column, ContainerAlign::Start, ContainerJustify::Start, 0, 0, None);
            line_text_col.add_content(Box::new(TextBox::new(&item.title, &self.title_font, &self.color)));
            let sub_text = format!("{}°C, {} {}km/h", (item.temp - 273.0) as u32, get_wind_dir(item.wind_dir), item.wind_speed as u32);
            line_text_col.add_content(Box::new(TextBox::new(&sub_text, &self.text_font, &self.color)));
            line_box.add_content(Box::new(line_text_col));

            top.add_content(Box::new(line_box));
        }
        Box::new(top)
    }
}
