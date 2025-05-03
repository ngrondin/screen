use std::io::Read;

use serde_json::Value;

use crate::models::weather::WeatherData;

use super::Provider;

pub struct OpenWeather {
    name: String,
    api_key: String,
    lat: f32,
    lon: f32
}

impl OpenWeather {
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
        OpenWeather { name: name.to_string(), api_key: api_key.to_string(), lat, lon }
    }
}

impl Provider for OpenWeather {
    fn provide(&mut self, data_store: &mut crate::data::DataStore) {
        let url = format!("https://api.openweathermap.org/data/2.5/weather?lat={}&lon={}&appid={}", self.lat, self.lon, self.api_key);
        if let Ok(mut res) = reqwest::blocking::get(&url) {
            let mut body_str = String::new();
            res.read_to_string(&mut body_str).unwrap();
            //println!("{}", &body_str);
            let in_data: Value = serde_json::from_str(&body_str).unwrap();
            let icon = in_data["weather"][0]["icon"].as_str().unwrap_or("").to_string();
            let data = WeatherData {
                title: in_data["weather"][0]["main"].as_str().unwrap_or("").to_string(),
                description: in_data["weather"][0]["description"].as_str().unwrap_or("").to_string(),
                icon: format!("https://openweathermap.org/img/wn/{}@2x.png", icon),
                temp: in_data["main"]["temp"].as_f64().unwrap_or(273.0),
                pressure:in_data["main"]["pressure"].as_u64().unwrap_or(1012),
                wind_dir:in_data["wind"]["deg"].as_u64().unwrap_or(0),
                wind_speed: in_data["wind"]["speed"].as_f64().unwrap_or(0.0),
                humidity: in_data["main"]["humidity"].as_u64().unwrap_or(0),
                cloud: in_data["clouds"]["all"].as_u64().unwrap_or(0),
                sunrise: in_data["sys"]["sunrise"].as_u64().unwrap_or(0),
                sunset: in_data["sys"]["sunset"].as_u64().unwrap_or(0)
            };
            let json_str = serde_json::to_string_pretty(&data).unwrap();
            data_store.store(&self.name, &json_str);     
        }
    }
}