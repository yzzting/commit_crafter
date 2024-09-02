use commit_crafter::{config, git_integration};
use std::fs;
use tempfile::tempdir;

#[test]
fn test_generate_config_toml() {
    let toml_str = config::generate_config_toml();
    assert!(toml_str.contains("openai_api_key = \"\""));
    assert!(toml_str.contains("openai_url = \"https://api.openai.com\""));
    assert!(toml_str.contains("openai_model = \"\""));
    assert!(toml_str.contains("user_language = \"en\""));
}

#[test]
fn test_write_config_toml() {
    let temp_dir = tempdir().unwrap();
    let file_path = temp_dir.path().join("config.toml");
    let config_str = config::generate_config_toml();
    if let Err(e) = config::write_config_to_toml(&config_str, &file_path) {
        eprintln!("Failed to write config.toml: {}", e);
    }

    let write_content = fs::read_to_string(&file_path).unwrap();
    assert_eq!(config_str, write_content);
}

#[test]
fn test_set_config_key() {
    let temp_dir = tempdir().unwrap();
    let file_path = temp_dir.path().join("config.toml");

    let config_str = config::generate_config_toml();
    if let Err(e) = config::write_config_to_toml(&config_str, &file_path) {
        eprintln!("Failed to write config.toml: {}", e);
    }

    config::set_config_key("openai_api_key", "test_key", &file_path).unwrap();

    let new_config_str = fs::read_to_string(&file_path).unwrap();
    assert!(new_config_str.contains("openai_api_key = \"test_key\""));

    temp_dir.close().unwrap();
}

#[test]
fn test_get_config_key() {
    let temp_dir = tempdir().unwrap();
    let file_path = temp_dir.path().join("config.toml");

    let config_str = config::generate_config_toml();
    if let Err(e) = config::write_config_to_toml(&config_str, &file_path) {
        eprintln!("Failed to write config.toml: {}", e);
    }

    let key = "openai_api_key";
    let value = config::get_config_key(&[key], &file_path).unwrap();
    assert_eq!(value, vec![""]);

    temp_dir.close().unwrap();
}

#[test]
fn test_list_config_keys() {
    let temp_dir = tempdir().unwrap();
    let file_path = temp_dir.path().join("config.toml");

    let config_str = config::generate_config_toml();
    if let Err(e) = config::write_config_to_toml(&config_str, &file_path) {
        eprintln!("Failed to write config.toml: {}", e);
    }

    let keys = [
        "openai_api_key",
        "openai_url",
        "openai_model",
        "user_language",
    ];
    let values = config::get_config_key(&keys, &file_path).unwrap();
    assert_eq!(values, vec!["", "https://api.openai.com", "gpt-4o-mini", "en"]);

    temp_dir.close().unwrap();
}

#[test]
fn test_exclude_path() {
    let files_to_exclude = vec![
        "Cargo.lock",
        "pakcage-lock.json",
        "pnpm-lock.yaml",
        "*.lock",
    ];

    let exclude_path: Vec<String> = files_to_exclude
        .iter()
        .map(|path| git_integration::exclude_from_diff(path))
        .collect();

    let expected_exclude_path = vec![
        ":(exclude)Cargo.lock",
        ":(exclude)pakcage-lock.json",
        ":(exclude)pnpm-lock.yaml",
        ":(exclude)*.lock",
    ];

    assert_eq!(exclude_path, expected_exclude_path);
}
