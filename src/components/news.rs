use std::rc::Rc;

use serde_json::Value;

use crate::{data::DataStore, fonts::{Font, FontFactory}, framebuffer::Color, layout::{containerbox::{ContainerAlign, ContainerBox, ContainerDir, ContainerJustify}, imagebox::ImageBox, textbox::TextBox}, utils::get_image, models::news::RssData};

use super::Component;


pub struct NewsUnit {
    data_name: String,
    title_font: Font,
    desc_font: Font,
    title_color: Color,
    desc_color: Color
}

impl NewsUnit {
    pub fn new(value: &Value, font_factory: Rc<FontFactory>) -> Self {
        let data_name = value["data"].as_str().or(Some("")).unwrap();
        let title_font = font_factory.get_font("DejaVuSans", 50.0).unwrap();
        let desc_font = font_factory.get_font("DejaVuSans", 30.0).unwrap();
        let title_color = Color::new(240, 240, 240);
        let desc_color = Color::new(200, 200, 200);
        NewsUnit { 
            data_name: data_name.to_string(),
            title_font,
            desc_font,
            title_color,
            desc_color
        }
    }
}

impl Component for NewsUnit {
    fn produce(&self, data_store: &DataStore) -> Box<dyn crate::layout::LayoutItem> {
        let mut top = ContainerBox::new(ContainerDir::Column, ContainerAlign::Start, ContainerJustify::Start, 1, 0, None);
        let data: RssData = data_store.load(&self.data_name);
        let mut count = 0;
        for channel in data.channels {
            let mut title_box = ContainerBox::new(ContainerDir::Row, ContainerAlign::Start, ContainerJustify::Start, 0, 20, None);
            let title_image_option = get_image(&channel.image.url);
            if let Some(title_image) = title_image_option {
                let title_image_box = ImageBox::new(Rc::new(title_image));
                title_box.add_content(Box::new(title_image_box));
            } else {
                title_box.add_content(Box::new(TextBox::new(&channel.title, &self.title_font, &self.title_color)));
            
            }
            top.add_content(Box::new(title_box));
            for item in channel.items {
                let mut item_box = ContainerBox::new(ContainerDir::Row, ContainerAlign::Start, ContainerJustify::Start, 0, 20, None);
                let image_option = get_image(&item.image.url);
                if let Some(image) = image_option {
                    let image_box = ImageBox::new_with_max_size(Rc::new(image), 300, 300);
                    item_box.add_content(Box::new(image_box));
                }
                let mut title_desc_box = ContainerBox::new(ContainerDir::Column, ContainerAlign::Start, ContainerJustify::Start, 0, 20, None);
                title_desc_box.add_content(Box::new(TextBox::new(&item.title, &self.title_font, &self.title_color)));
                title_desc_box.add_content(Box::new(TextBox::new(&item.description, &self.desc_font, &self.desc_color)));
                item_box.add_content(Box::new(title_desc_box));
                top.add_content(Box::new(item_box));
                count += 1;
                if count > 3 {
                    break;
                }
            }
        }
        Box::new(top)
    }
}