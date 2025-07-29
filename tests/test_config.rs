use commit_crafter::{config, git_integration};
use std::env;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_generate_config_toml() {
    let toml_str = config::generate_config_toml();
    assert!(toml_str.contains("openai_api_key = \"\""));
    assert!(toml_str.contains("openai_url = \"https://api.openai.com\""));
    assert!(toml_str.contains("openai_model = \"gpt-4o-mini\""));
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
    assert_eq!(
        values,
        vec!["", "https://api.openai.com", "gpt-4o-mini", "en"]
    );

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

#[test]
fn test_set_get_config_key() {
    if env::var("GITHUB_ACTIONS").is_ok() {
        eprintln!("Skipping test in GitHub Actions environment");
        return;
    }
    let temp_dir = tempdir().unwrap();
    let config_path = temp_dir.path().join("config.toml");
    let prompt_path = temp_dir.path().join("prompt.toml");

    // generate config.toml
    let config_str = config::generate_config_toml();
    if let Err(e) = config::write_config_to_toml(&config_str, &config_path) {
        eprintln!("Failed to write config.toml: {}", e);
    }

    // generate prompt.toml
    config::move_prompt_toml(&prompt_path);

    // test set_config_key
    let result = config::set_config_key("openai_api_key", "test_api_key", &config_path);
    assert!(result.is_ok());

    // test get_config_key
    let result = config::get_config_key(&["openai_api_key"], &config_path);
    assert!(result.is_ok());
    assert_eq!(result.unwrap()[0], "test_api_key");

    // cleanup
    temp_dir.close().unwrap();
}

#[test]
fn test_ensure_config_initialized() {
    if env::var("GITHUB_ACTIONS").is_ok() {
        eprintln!("Skipping test in GitHub Actions environment");
        return;
    }

    let temp_dir = tempdir().unwrap();
    let config_dir_path = temp_dir.path().join("test_project_config");

    assert!(!config_dir_path.exists());

    let result = config::ensure_config_initialized(&config_dir_path);
    assert!(result.is_ok());

    assert!(config_dir_path.exists());
    assert!(config_dir_path.is_dir());

    let config_file = config_dir_path.join("config.toml");
    let prompt_file = config_dir_path.join("prompt.toml");

    assert!(config_file.exists());
    assert!(prompt_file.exists());

    let keys = [
        "openai_api_key",
        "openai_url",
        "openai_model",
        "user_language",
    ];
    let result = config::get_config_key(&keys, &config_file);
    assert!(result.is_ok());

    let values = result.unwrap();
    assert_eq!(values.len(), 4);
    assert_eq!(values[1], "https://api.openai.com"); // openai_url
    assert_eq!(values[2], "gpt-4o-mini"); // openai_model
    assert_eq!(values[3], "en"); // user_language

    // cleanup
    temp_dir.close().unwrap();
}

#[test]
fn test_ensure_config_initialized_existing_config() {
    if env::var("GITHUB_ACTIONS").is_ok() {
        eprintln!("Skipping test in GitHub Actions environment");
        return;
    }

    let temp_dir = tempdir().unwrap();
    let config_dir_path = temp_dir.path().join("existing_project_config");

    std::fs::create_dir_all(&config_dir_path).unwrap();

    let config_file = config_dir_path.join("config.toml");
    let config_str = config::generate_config_toml();
    config::write_config_to_toml(&config_str, &config_file).unwrap();

    config::set_config_key("openai_api_key", "existing_key", &config_file).unwrap();

    let result = config::ensure_config_initialized(&config_dir_path);
    assert!(result.is_ok());

    let result = config::get_config_key(&["openai_api_key"], &config_file);
    assert!(result.is_ok());
    assert_eq!(result.unwrap()[0], "existing_key");

    // cleanup
    temp_dir.close().unwrap();
}

#[test]
fn test_set_get_config_key_error() {
    if env::var("GITHUB_ACTIONS").is_ok() {
        eprintln!("Skipping test in GitHub Actions environment");
        return;
    }
    let temp_dir = tempdir().unwrap();
    let config_path = temp_dir.path().join("config.toml");

    // generate config.toml
    let config_str = config::generate_config_toml();
    if let Err(e) = config::write_config_to_toml(&config_str, &config_path) {
        eprintln!("Failed to write config.toml: {}", e);
    }

    // test set_config_key with invalid key
    let result = config::set_config_key("invalid_key", "invalid_value", &config_path);
    assert!(result.is_err());

    // test get_config_key with invalid key
    let result = config::get_config_key(&["invalid_key"], &config_path);
    assert!(result.is_err());

    // cleanup
    temp_dir.close().unwrap();
}

#[test]
fn test_move_prompt_toml() {
    if env::var("GITHUB_ACTIONS").is_ok() {
        eprintln!("Skipping test in GitHub Actions environment");
        return;
    }
    let temp_dir = tempdir().unwrap();
    let prompt_path = temp_dir.path().join("prompt.toml");

    // Test move_prompt_toml
    config::move_prompt_toml(&prompt_path);

    // verify the prompt.toml file exists
    assert!(prompt_path.exists());

    // verify the prompt.toml content
    let prompt_file_content = std::fs::read_to_string(&prompt_path).unwrap();
    assert!(prompt_file_content.contains("prompt_zh"));

    // cleanup
    temp_dir.close().unwrap();
}

#[test]
fn test_get_language() {
    if env::var("GITHUB_ACTIONS").is_ok() {
        eprintln!("Skipping test in GitHub Actions environment");
        return;
    }
    let temp_dir = tempdir().unwrap();
    let prompt_path = temp_dir.path().join("prompt.toml");

    // Test move_prompt_toml
    config::move_prompt_toml(&prompt_path);

    // test get_language with different language
    let en_prompt = config::get_language("en", &prompt_path);
    assert!(en_prompt.contains("English"));

    let zh_prompt = config::get_language("zh", &prompt_path);
    assert!(zh_prompt.contains("简体中文"));

    // cleanup
    temp_dir.close().unwrap();
}
