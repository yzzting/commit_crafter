mod config;
mod git_integration;
mod install;
mod llm;

use clap::Command;

fn main() {
    let matches = Command::new("commit crafter")
        .version("0.1.0")
        .author("ZenYang <yzzting@gmail.com")
        .subcommand(
            Command::new("install")
                .about("Install the pre-commit hook")
                .version("0.1.0")
                .author("ZenYang <yzzting@gmail.com>"),
        )
        .get_matches();
    match matches.subcommand() {
        Some(("install", _sub_matches)) => {
            install::install_commit_msg_hook();
        }
        None => match git_integration::run_git_diff() {
            Ok(output) => {
                if output.is_empty() {
                    eprintln!("Error: No changes to commit");
                    std::process::exit(1);
                }
                llm::openai::openai_request(&output).unwrap();
            }
            Err(e) => eprintln!("Error: {}", e),
        },
        _ => (),
    }
}
