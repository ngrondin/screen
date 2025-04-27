use std::rc::Rc;

use datetime::{DatePiece, LocalDateTime, TimePiece};
use serde_json::Value;

use crate::{data::DataStore, fonts::{Font, FontFactory}, framebuffer::Color, layout::{containerbox::{ContainerAlign, ContainerBox, ContainerDir}, textbox::TextBox}, utils::{get_month_name, get_weekday_name}};

use super::Component;


pub struct DateTimeUnit {
    show_date: bool,
    time_font: Font,
    date_font: Font,
    color: Color
}

impl DateTimeUnit {
    pub fn new(value: &Value, font_factory: Rc<FontFactory>) -> Self {
        let show_date = value["showdate"].as_bool().unwrap_or(true);
        let time_font = font_factory.get_font("DejaVuSans", 200.0).unwrap();
        let date_font = font_factory.get_font("DejaVuSans", 40.0).unwrap();
        let color = Color::new(240, 240, 240);
        DateTimeUnit { 
            show_date,
            time_font,
            date_font,
            color
        }
    }
}

impl Component for DateTimeUnit {
    fn produce(&self, _data_store: &DataStore) -> Box<dyn crate::layout::LayoutItem> {
        let now = LocalDateTime::now();
        let time_str = format!("{}:{}", now.time().hour(), now.time().minute());
        let date_str = format!("{}, {} {}", get_weekday_name(now.date().weekday()), now.date().day(), get_month_name(now.date().month()));
        let mut top = ContainerBox::new(ContainerDir::Column, ContainerAlign::Center, 0, 0, None);
        top.add_content(Box::new(TextBox::new(&time_str, &self.time_font, &self.color)));
        if self.show_date {
            top.add_content(Box::new(TextBox::new(&date_str, &self.date_font, &self.color)));
        }
        Box::new(top)
    }
}