use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use toml;

#[derive(Deserialize, Serialize)]
struct Config {
    openai_api_key: String,
    openai_url: String,
    openai_model: String,
    user_language: String,
}

#[derive(Deserialize, Serialize)]
struct PromptConfig {
    prompt_zh: String,
    prompt_en: String,
    prompt_jp: String,
    prompt_zh_tw: String,
}

pub const VALID_OPENAI_API_KEY: &str = "openai_api_key";
pub const VALID_OPENAI_URL: &str = "openai_url";
pub const VALID_OPENAI_MODEL: &str = "openai_model";
pub const VALID_USER_LANGUAGE: &str = "user_language";

pub fn validate_config_key(key: &str) -> Result<&str, &'static str> {
    match key {
        VALID_OPENAI_API_KEY | VALID_OPENAI_URL | VALID_OPENAI_MODEL => Ok(key),
        VALID_USER_LANGUAGE => Ok(key),
        _ => Err("Invalid configuration key"),
    }
}

pub fn get_config_key<P: AsRef<Path>>(keys: &[&str], path: P) -> Result<Vec<String>, &'static str> {
    let config_file = fs::read_to_string(path.as_ref()).expect("Could not read config file");
    let config: Config = toml::from_str(&config_file).expect("Could not parse config file");

    let mut result = Vec::new();

    for key in keys {
        match validate_config_key(key) {
            Ok(vaild_key) => {
                let value = match vaild_key {
                    VALID_OPENAI_API_KEY => config.openai_api_key.clone(),
                    VALID_OPENAI_URL => config.openai_url.clone(),
                    VALID_OPENAI_MODEL => config.openai_model.clone(),
                    VALID_USER_LANGUAGE => config.user_language.clone(),
                    _ => panic!("Invalid configuration key"),
                };
                result.push(value);
            }
            Err(_) => return Err("Invalid configuration key"),
        }
    }
    Ok(result)
}

pub fn set_config_key<P: AsRef<Path> + Clone>(
    key: &str,
    value: &str,
    path: P,
) -> Result<(), Box<dyn std::error::Error>> {
    let config_file = fs::read_to_string(path.as_ref()).expect("Could not read config file");
    let mut config: Config = toml::from_str(&config_file).expect("Could not parse config file");

    let key = validate_config_key(key).expect("Invalid configuration key");
    match key {
        VALID_OPENAI_API_KEY => config.openai_api_key = value.to_string(),
        VALID_OPENAI_URL => config.openai_url = value.to_string(),
        VALID_OPENAI_MODEL => config.openai_model = value.to_string(),
        VALID_USER_LANGUAGE => config.user_language = value.to_string(),
        _ => panic!("Invalid configuration key"),
    }
    let new_config = toml::to_string(&config).expect("Could not serialize config");
    fs::write(path, new_config).expect("Could not write to config file");

    Ok(())
}

pub fn get_language(user_language: &str) -> String {
    let prompt_file = fs::read_to_string("prompt.toml").expect("Could not read prompt config file");
    let prompt_config: PromptConfig =
        toml::from_str(&prompt_file).expect("Could not parse prompt config file");

    match user_language {
        "zh" => prompt_config.prompt_zh.clone(),
        "en" => prompt_config.prompt_en.clone(),
        "jp" => prompt_config.prompt_jp.clone(),
        "zh_tw" => prompt_config.prompt_zh_tw.clone(),
        _ => panic!("Invalid user language"),
    }
}

pub fn generate_config_toml() -> String {
    let config = Config {
        openai_api_key: "".to_string(),
        openai_url: "https://api.openai.com".to_string(),
        openai_model: "".to_string(),
        user_language: "en".to_string(),
    };
    toml::to_string(&config).expect("Could not serialize config")
}

pub fn write_config_to_toml(config_toml: &str, path: &Path) {
    fs::write(path, config_toml).expect("Could not write to config file");
}
