use std::io::Read;

use datetime::LocalDateTime;
use serde_json::Value;

use crate::models::weather::{WeatherForecastData, WeatherForecastItemData};

use super::Provider;

pub struct OpenWeatherForecast {
    name: String,
    api_key: String,
    lat: f32,
    lon: f32
}

impl OpenWeatherForecast {
    pub fn new(name: &str, value: &Value) -> Self {
        let api_key = match value["apikey"].as_str() {
            Some(api_key_str) => api_key_str,
            None => ""
        };
        let lat = match value["lat"].as_f64() {
            Some(num) => num,
            None => 0.0
        } as f32;
        let lon = match value["lon"].as_f64() {
            Some(num) => num,
            None => 0.0
        } as f32;
        OpenWeatherForecast { name: name.to_string(), api_key: api_key.to_string(), lat, lon }
    }
}

impl Provider for OpenWeatherForecast {
    fn provide(&mut self, data_store: &mut crate::data::DataStore) {
        let url = format!("https://api.openweathermap.org/data/2.5/forecast?lat={}&lon={}&appid={}", self.lat, self.lon, self.api_key);
        let mut res = reqwest::blocking::get(&url).unwrap();
        let mut body_str = String::new();
        res.read_to_string(&mut body_str).unwrap();
        //println!("{}", &body_str);
        let in_data: Value = serde_json::from_str(&body_str).unwrap();
        let in_list = in_data["list"].as_array().unwrap();
        let mut list: Vec<WeatherForecastItemData> = vec![];
        for in_item in in_list {
            let ts = in_item["dt"].as_u64().unwrap_or(0) as i64;
            let forecast_time = LocalDateTime::at(ts);
            if forecast_time.time().to_seconds() == 43200 {
                let icon = in_item["weather"][0]["icon"].as_str().unwrap_or("").to_string();
                let item = WeatherForecastItemData {
                    ts,
                    title: in_item["weather"][0]["main"].as_str().unwrap_or("").to_string(),
                    icon: format!("https://openweathermap.org/img/wn/{}.png", icon),
                    temp: in_item["main"]["temp"].as_f64().unwrap_or(273.0),
                    wind_dir: in_item["wind"]["deg"].as_u64().unwrap_or(0),
                    wind_speed: in_item["wind"]["speed"].as_f64().unwrap_or(0.0)
                };
                list.push(item);                    
            }
        }
        let data = WeatherForecastData { list };
        let json_str = serde_json::to_string_pretty(&data).unwrap();
        data_store.store(&self.name, &json_str); 
    }
}