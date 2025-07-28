use reqwest::blocking::Client;
use serde_json::{json, Value};
use std::io::{Error, ErrorKind, Result};

use crate::config::{
    get_config_key, get_language, VALID_OPENAI_API_KEY, VALID_OPENAI_MODEL, VALID_OPENAI_URL,
    VALID_USER_LANGUAGE,
};

pub fn openai_request(diff_content: &str, commit_history: &[String], path: &str) -> Result<()> {
    let keys = [
        VALID_OPENAI_API_KEY,
        VALID_OPENAI_URL,
        VALID_OPENAI_MODEL,
        VALID_USER_LANGUAGE,
    ];
    let mut openai_api_key = String::new();
    let mut openai_url = String::new();
    let mut openai_model = String::new();
    let mut user_language = String::new();
    let config_dir = format!("{}/config.toml", path);
    let prompt = format!("{}/prompt.toml", path);
    match get_config_key(&keys, config_dir) {
        Ok(values) => {
            openai_api_key = values[0].clone();
            openai_url = values[1].clone();
            openai_model = values[2].clone();
            user_language = values[3].clone();
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
    let base_prompt = get_language(user_language.as_str(), prompt);

    // build user message with recent commit messages
    let mut user_message = String::new();

    if !commit_history.is_empty() {
        user_message.push_str("Recent commit messages for reference:\n");
        for (i, commit) in commit_history.iter().enumerate() {
            user_message.push_str(&format!("{}. {}\n", i + 1, commit));
        }
        user_message.push_str("\n---\n\n");
    }

    user_message.push_str("Git diff content:\n");
    user_message.push_str(diff_content);

    let client = Client::new();
    let response = client
        .post(format!("{}/v1/chat/completions", openai_url))
        .header("Authorization", format!("Bearer {}", openai_api_key))
        .json(&json!({
            "model": openai_model,
            "messages": [
                {
                    "role": "system",
                    "content": base_prompt
                },
                {
                    "role": "user",
                    "content": user_message
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
