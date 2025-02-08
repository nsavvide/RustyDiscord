use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct ColorScheme {
    pub border: String,
    pub text: String,
    pub highlight: String,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub colorschemes: HashMap<String, ColorScheme>,
}

impl Config {
    pub fn load() -> Self {
        let config_str = include_str!("../config.toml");
        toml::from_str(config_str).expect("Failed to parse config file")
    }
}
