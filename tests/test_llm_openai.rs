use commit_crafter::{config, llm};
use tempfile::tempdir;

#[test]
fn test_openai_request_by_no_config() {
    let temp_dir = tempdir().unwrap();
    let file_path = temp_dir.path().join("config.toml");
    let config_str = config::generate_config_toml();
    config::write_config_to_toml(&config_str, &file_path);

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
    // run openai_request
    let result = llm::openai::openai_request("diff_content", "config.toml");

    match result {
        Ok(_) => assert!(true),
        Err(e) => panic!("Expected Ok, got {:?}", e),
    }
}
