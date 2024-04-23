use reqwest::blocking::Client;
use reqwest::Error;
use serde_json::{json, Value};

use crate::config::{get_config_key, get_language, VALID_OPENAI_API_KEY, VALID_OPENAI_URL};

pub fn openai_request(diff_content: &str) -> Result<(), Error> {
    let openai_api_key = get_config_key(VALID_OPENAI_API_KEY);
    let openai_url = get_config_key(VALID_OPENAI_URL);
    if openai_api_key.is_empty() || openai_url.is_empty() {
        eprintln!("Error: OpenAI API key or URL is empty");
        std::process::exit(1);
    }
    let prompt = get_language();
    let client = Client::new();
    let response = client
        .post(format!("{}/v1/chat/completions", openai_url))
        .header("Authorization", format!("Bearer {}", openai_api_key))
        .json(&json!({
            "model": "gpt-3.5-turbo",
            "messages": [
                {
                    "role": "system",
                    "content": prompt
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
            println!("{}", text);
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
