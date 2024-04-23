use serde::{Deserialize, Serialize};
use std::fs;
use toml;

#[derive(Deserialize, Serialize)]
struct Config {
    openai_api_key: String,
    openai_url: String,
    default_language: String,
    user_language: String,
}

pub const VALID_OPENAI_API_KEY: &str = "openai_api_key";
pub const VALID_OPENAI_URL: &str = "openai_url";
const VALID_DEFAULT_LANGUAGE: &str = "default_language";
const VALID_USER_LANGUAGE: &str = "user_language";

pub fn validate_config_key(key: &str) -> Result<&str, &'static str> {
    match key {
        VALID_OPENAI_API_KEY | VALID_OPENAI_URL => Ok(key),
        VALID_DEFAULT_LANGUAGE | VALID_USER_LANGUAGE => Ok(key),
        _ => Err("Invalid configuration key"),
    }
}

pub fn get_config_key(key: &str) -> String {
    let config_file = fs::read_to_string("config.toml").expect("Could not read config file");
    let config: Config = toml::from_str(&config_file).expect("Could not parse config file");

    let key = validate_config_key(key).expect("Invalid configuration key");
    match key {
        VALID_OPENAI_API_KEY => config.openai_api_key,
        VALID_OPENAI_URL => config.openai_url,
        VALID_DEFAULT_LANGUAGE => config.default_language,
        VALID_USER_LANGUAGE => config.user_language,
        _ => panic!("Invalid configuration key"),
    }
}

pub fn set_config_key(key: &str, value: &str) -> Result<(), Box<dyn std::error::Error>> {
    let config_file = fs::read_to_string("config.toml").expect("Could not read config file");
    let mut config: Config = toml::from_str(&config_file).expect("Could not parse config file");

    let key = validate_config_key(key).expect("Invalid configuration key");
    match key {
        VALID_OPENAI_API_KEY => config.openai_api_key = value.to_string(),
        VALID_OPENAI_URL => config.openai_url = value.to_string(),
        VALID_DEFAULT_LANGUAGE => config.default_language = value.to_string(),
        VALID_USER_LANGUAGE => config.user_language = value.to_string(),
        _ => panic!("Invalid configuration key"),
    }
    let new_config = toml::to_string(&config).expect("Could not serialize config");
    fs::write("config.toml", new_config).expect("Could not write to config file");

    Ok(())
}

pub fn get_language() -> String {
    let default_language = get_config_key(VALID_DEFAULT_LANGUAGE);
    let user_language = get_config_key(VALID_USER_LANGUAGE);
    if user_language.is_empty() {
        default_language
    } else {
        user_language
    }
}
