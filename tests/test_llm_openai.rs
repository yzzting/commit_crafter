use commit_crafter::{config, llm};
use std::env;
use tempfile::tempdir;

#[test]
fn test_openai_request_by_no_config() {
    if env::var("GITHUB_ACTIONS").is_ok() {
        eprintln!("Skipping test in GitHub Actions environment");
        return;
    }
    let temp_dir = tempdir().unwrap();
    let prompt_dir = temp_dir.path().join("prompt.toml");
    let file_path = temp_dir.path();
    // generate prompt.toml
    config::move_prompt_toml(&prompt_dir);
    let config_str = config::generate_config_toml();
    if let Err(e) = config::write_config_to_toml(&config_str, &file_path.join("config.toml")) {
        eprintln!("Failed to write config.toml: {}", e);
    }

    // run openai_request with empty commit history
    let commit_history = vec![];
    let result =
        llm::openai::openai_request("diff_content", &commit_history, file_path.to_str().unwrap());

    match result {
        Ok(_) => panic!("Expected an error of 'NotFound', got {:?}", result),
        Err(e) => assert_eq!(e.to_string(), "OpenAI API key or URL is empty"),
    }

    // clean up
    temp_dir.close().unwrap();
}

#[test]
fn test_openai_request() {
    if env::var("GITHUB_ACTIONS").is_ok() {
        eprintln!("Skipping test in GitHub Actions environment");
        return;
    }
    let home_dir = env::var("HOME").expect("Error getting home directory");
    let config_dir = format!("{}/.config/commit_crafter", home_dir);
    let prompt_dir = format!("{}/.config/commit_crafter/prompt.toml", home_dir);
    // generate prompt.toml
    config::move_prompt_toml(&prompt_dir);

    // run openai_request with sample commit history
    let commit_history = vec![
        "feat: add new feature".to_string(),
        "fix: resolve bug in authentication".to_string(),
        "docs: update README".to_string(),
    ];
    let result = llm::openai::openai_request("diff_content", &commit_history, &config_dir);

    match result {
        Ok(_) => assert!(true),
        Err(e) => panic!("Expected Ok, got {:?}", e),
    }
}

#[test]
fn test_openai_request_with_empty_history() {
    if env::var("GITHUB_ACTIONS").is_ok() {
        eprintln!("Skipping test in GitHub Actions environment");
        return;
    }
    let home_dir = env::var("HOME").expect("Error getting home directory");
    let config_dir = format!("{}/.config/commit_crafter", home_dir);
    let prompt_dir = format!("{}/.config/commit_crafter/prompt.toml", home_dir);
    // generate prompt.toml
    config::move_prompt_toml(&prompt_dir);

    // run openai_request with empty commit history
    let commit_history = vec![];
    let result = llm::openai::openai_request("diff_content", &commit_history, &config_dir);

    match result {
        Ok(_) => assert!(true),
        Err(e) => panic!("Expected Ok, got {:?}", e),
    }
}
