use std::rc::Rc;

use serde_json::Value;

use crate::{components::{container::Container, image::ImageUnit, text::TextUnit, Component}, fonts::FontFactory, utils::{list_folder_configs, load_config}};

use super::{datetime::DateTimeUnit, news::NewsUnit, openweather::WeatherUnit, openweatherforecast::WeatherForecastUnit, Page};


pub struct PageFactory {
    folder: String,
    font_factory: Rc<FontFactory>,
}

impl PageFactory {
    pub fn new(f: &str) -> Self {
        let ff = FontFactory::new();
        PageFactory { folder: f.to_string(), font_factory: Rc::new(ff) }
    }

    pub fn list_pages(&self) -> Vec<String> {
        list_folder_configs(&self.folder)
    }

    pub fn load_page(&self, name: &str) -> Page {
        let config = load_config(&self.folder, name);
        let top_component = self.recursive_comp_create(config);
        let page = Page::new(top_component);
        page
    }

    fn recursive_comp_create(&self, value: Value) -> Box<dyn Component> {
        match value["type"].as_str() {
            Some(t) => {
                match t {
                    "container" => {
                        let mut container = Container::new(&value);
                        if let Some(content_array) = value["content"].as_array() {
                            for array_item in content_array.iter() {
                                let child = self.recursive_comp_create(array_item.clone());
                                container.add_child(child);
                            }
                        }
                        Box::new(container)
                    },
                    "text" => Box::new(TextUnit::new(&value, self.font_factory.clone())),
                    "image" => Box::new(ImageUnit::new(&value)),
                    "news" => Box::new(NewsUnit::new(&value, self.font_factory.clone())),
                    "weather" => Box::new(WeatherUnit::new(&value, self.font_factory.clone())),
                    "weatherforecast" => Box::new(WeatherForecastUnit::new(&value, self.font_factory.clone())),
                    "datetime" => Box::new(DateTimeUnit::new(&value, self.font_factory.clone())),
                    _ => Box::new(Container::new(&value))
                }
            }
            None => Box::new(Container::new(&value))
        }
    }
 }