use reqwest::blocking::Client;
use reqwest::Error;
use serde_json::{json, Value};

use crate::config::{get_config_key, ConfigKey};

pub fn openai_request(diff_content: &str) -> Result<(), Error> {
    let openai_key = get_config_key(ConfigKey::OpenaiApiKey);
    let client = Client::new();
    let response = client
        .post("https://aihubmix.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", openai_key))
        .json(&json!({
            "model": "gpt-3.5-turbo",
            "messages": [
                {
                    "role": "system",
                    "content": "Please generate a commit message based on the following git diff:"
                },
                {
                    "role": "user",
                    "content": diff_content
                }
            ]
        }))
        .send()?;

    if response.status().is_success() {
        let response_json: Value = response.json()?;
        if let Some(text) = response_json["choices"][0]["message"]["content"].as_str() {
            println!("output: {}", text);
        } else {
            eprintln!("Error: Could not parse response");
        }
    } else {
        eprintln!(
            "Error: Request failed with status code: {}",
            response.status()
        );
    }

    Ok(())
}
