use serde::Deserialize;
use std::fs;
use toml;

#[derive(Deserialize)]
struct Config {
    OPEN_AI_KEY: String,
}

pub enum ConfigKey {
    OpenaiApiKey,
}

pub fn get_config_key(key: ConfigKey) -> String {
    let config_file = fs::read_to_string("config.toml").expect("Could not read config file");
    let config: Config = toml::from_str(&config_file).expect("Could not parse config file");

    match key {
        ConfigKey::OpenaiApiKey => config.OPEN_AI_KEY,
    }
}
