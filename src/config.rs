use serde::Deserialize;
use std::fs;
use toml;

#[derive(Deserialize)]
struct Config {
    open_ai_key: String,
    open_ai_url: String,
}

pub enum ConfigKey {
    OpenaiApiKey,
    OpenaiApiUrl,
}

pub fn get_config_key(key: ConfigKey) -> String {
    let config_file = fs::read_to_string("config.toml").expect("Could not read config file");
    let config: Config = toml::from_str(&config_file).expect("Could not parse config file");

    match key {
        ConfigKey::OpenaiApiKey => config.open_ai_key,
        ConfigKey::OpenaiApiUrl => config.open_ai_url,
    }
}
