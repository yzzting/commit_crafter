use commit_crafter::config;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_generate_config_toml() {
    let toml_str = config::generate_config_toml();
    assert!(toml_str.contains("openai_api_key = \"\""));
    assert!(toml_str.contains("openai_url = \"\""));
    assert!(toml_str.contains("openai_model = \"\""));
    assert!(toml_str.contains("user_language = \"en\""));
} 

#[test]
fn test_write_config_toml() {
    let temp_dir = tempdir().unwrap();
    let file_path = temp_dir.path().join("config.toml");
    let config_str = config::generate_config_toml();
    config::write_config_to_toml(&config_str, &file_path);

    let write_content = fs::read_to_string(&file_path).unwrap();
    assert_eq!(config_str, write_content);
}