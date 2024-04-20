mod config;
mod git_integration;
mod llm;

fn main() {
    // git_integration::run_git_command();
    match git_integration::run_git_diff() {
        Ok(output) => {
            println!("Git Diff Output: {}", output);
            llm::openai::openai_request(&output).unwrap();
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
