use reqwest::blocking::Client;
use serde_json::{json, Value};
use std::io::{Error, ErrorKind, Result};

use crate::config::{
    get_config_key, VALID_OPENAI_API_KEY, VALID_OPENAI_MODEL, VALID_OPENAI_URL, VALID_USER_LANGUAGE,
};

pub fn openai_request(diff_content: &str) -> Result<()> {
    let keys = [
        VALID_OPENAI_API_KEY,
        VALID_OPENAI_URL,
        VALID_OPENAI_MODEL,
        VALID_USER_LANGUAGE,
    ];
    let mut openai_api_key = String::new();
    let mut openai_url = String::new();
    let mut openai_model = String::new();
    let mut prompt = String::new();
    match get_config_key(&keys) {
        Ok(values) => {
            openai_api_key = values[0].clone();
            openai_url = values[1].clone();
            openai_model = values[2].clone();
            prompt = values[3].clone();
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }

    if openai_api_key.is_empty() || openai_url.is_empty() {
        eprintln!("Error: OpenAI API key or URL is empty");
        return Err(Error::new(
            ErrorKind::NotFound,
            "OpenAI API key or URL is empty",
        ));
    }
    let client = Client::new();
    let response = client
        .post(format!("{}/v1/chat/completions", openai_url))
        .header("Authorization", format!("Bearer {}", openai_api_key))
        .json(&json!({
            "model": openai_model,
            "messages": [
                {
                    "role": "system",
                    "content": prompt
                },
                {
                    "role": "user",
                    "content": diff_content
                }
            ],
            "max_tokens": 60
        }))
        .send()
        .expect("Error sending request");

    if response.status().is_success() {
        let response_json: Value = response.json().expect("Error parsing response");
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
