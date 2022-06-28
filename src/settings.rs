use config::{Config, ConfigError, Environment};
use dotenv::dotenv;
use lazy_static::lazy_static;
use serde_derive::Deserialize;
use std::sync::RwLock;

#[derive(Debug, Deserialize)]
pub struct Database {
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub database: Database,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        dotenv().ok();
        let s = Config::builder()
            .add_source(Environment::with_prefix("HR"))
            .build()?;
        s.try_deserialize()
    }
}

lazy_static! {
    pub static ref SETTINGS: RwLock<Settings> = RwLock::new(Settings::new().unwrap());
}
