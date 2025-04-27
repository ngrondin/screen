
use serde_json::Value;

use crate::utils::{list_folder_configs, load_config};

use super::{openweather::OpenWeather, openweatherforecast::OpenWeatherForecast, rss::RSSProvider, NoopProvider, Provider};

pub struct ProviderFactory {
    folder: String
}

impl ProviderFactory {
    pub fn new(path: &str) -> Self {
        ProviderFactory { folder: path.to_string() }
    }

    pub fn list_providers(&self) -> Vec<String> {
        list_folder_configs(&self.folder)
    }

    pub fn load_provider(&self, name: &str) -> Box<dyn Provider> {
        let config = load_config(&self.folder, name);
        let provider = self.create(name, config);
        provider
    }

    fn create(&self, name: &str, value: Value) -> Box<dyn Provider> {
        match value["type"].as_str() {
            Some(t) => {
                match t {
                    "rss" => Box::new(RSSProvider::new(name, &value)),
                    "openweather" => Box::new(OpenWeather::new(name, &value)),
                    "openweatherforecast" => Box::new(OpenWeatherForecast::new(name, &value)),
                    _ => Box::new(NoopProvider::new())
                }
            }
            None => Box::new(NoopProvider::new())
        }
    }
}