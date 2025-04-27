use crate::data::DataStore;

pub mod factory;
pub mod rss;
pub mod openweather;
pub mod openweatherforecast;

pub trait Provider {
    fn provide(&mut self, data_store: &mut DataStore);
}


pub struct NoopProvider {
}

impl NoopProvider {
    fn new() -> Self {
        NoopProvider {  }
    }
}

impl Provider for NoopProvider {
    fn provide(&mut self, _data_store: &mut DataStore) {
    }
}