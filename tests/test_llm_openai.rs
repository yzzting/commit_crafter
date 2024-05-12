use commit_crafter::{config, llm};
use std::env;
use tempfile::tempdir;

#[test]
fn test_openai_request_by_no_config() {
    let temp_dir = tempdir().unwrap();
    let file_path = temp_dir.path().join("config.toml");
    let config_str = config::generate_config_toml();
    if let Err(e) = config::write_config_to_toml(&config_str, &file_path) {
        eprintln!("Failed to write config.toml: {}", e);
    }

    // run openai_request
    let result = llm::openai::openai_request("diff_content", file_path.to_str().unwrap());

    match result {
        Ok(_) => panic!("Expected an error of 'NotFound', got {:?}", result),
        Err(e) => assert_eq!(e.to_string(), "OpenAI API key or URL is empty"),
    }

    // clean up
    temp_dir.close().unwrap();
}

#[test]
fn test_openai_request() {
    let home_dir = env::var("HOME").expect("Error getting home directory");
    let config_dir = format!("{}/.config/commit_crafter/config.toml", home_dir);
    // run openai_request
    let result = llm::openai::openai_request("diff_content", &config_dir);

    match result {
        Ok(_) => assert!(true),
        Err(e) => panic!("Expected Ok, got {:?}", e),
    }
}
