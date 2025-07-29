extern crate commit_crafter;
use std::collections::hash_map::DefaultHasher;
use std::env;
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::Path;

use commit_crafter::{config, git_integration, install, llm, uninstall};

use clap::{arg, Command};

fn main() {
    let matches = Command::new("commit crafter")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .subcommand(
            Command::new("install")
                .about("Install the pre-commit hook")
                .version(env!("CARGO_PKG_VERSION"))
                .author(env!("CARGO_PKG_AUTHORS")),
        )
        .subcommand(
            Command::new("config")
                .about("Configure settings")
                .version(env!("CARGO_PKG_VERSION"))
                .author(env!("CARGO_PKG_AUTHORS"))
                .subcommand(
                    Command::new("set")
                        .about("Set a configuration option")
                        .arg(arg!(<KEY> "The configuration key to set"))
                        .arg(arg!(<VALUE> "The value to set the key to")),
                )
                .subcommand(
                    Command::new("get")
                        .about("Get a configuration option")
                        .arg(arg!(<KEY> "The configuration key to get")),
                )
                .subcommand(Command::new("list").about("List all configuration options")),
        )
        .subcommand(
            Command::new("uninstall")
                .about("Uninstall the pre-commit hook")
                .version(env!("CARGO_PKG_VERSION"))
                .author(env!("CARGO_PKG_AUTHORS")),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("install", _sub_matches)) => {
            let _ = install::install_commit_msg_hook();
        }
        Some(("uninstall", _sub_matches)) => {
            let _ = uninstall::uninstall_commit_msg_hook();
        }
        Some(("config", sub_matches)) => {
            handle_config_subcommand(sub_matches);
        }
        _none => match git_integration::run_git_diff() {
            Ok(output) => {
                if output.is_empty() {
                    eprintln!("Error: No changes to commit");
                    std::process::exit(1);
                }
                let config_dir = get_config_dir("");

                // Initialize config if needed
                if let Err(e) = config::ensure_config_initialized(&config_dir) {
                    eprintln!("Warning: Failed to initialize config: {}", e);
                }

                // get recent 5 commit messages as reference
                let commit_history = match git_integration::get_recent_commits(5) {
                    Ok(commits) => commits,
                    Err(e) => {
                        eprintln!("Warning: Failed to get recent commits: {}", e);
                        Vec::new() // if failed to get recent commits, use empty vector
                    }
                };

                llm::openai::openai_request(&output, &commit_history, &config_dir).unwrap();
            }
            Err(e) => eprintln!("Error: {}", e),
        },
    }
}

fn get_config_dir(config_url: &str) -> String {
    let home_dir = env::var("HOME").expect("Error getting home directory");
    let base_config_dir = format!("{}/.config/commit_crafter", home_dir);

    // Try to get git root directory for project-specific config
    match git_integration::get_git_root_dir() {
        Ok(git_root) => {
            // Create a hash of the git root path to create a unique config directory for this project
            let mut hasher = DefaultHasher::new();
            git_root.hash(&mut hasher);
            let project_hash = hasher.finish();

            let project_name = git_root
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("unknown");

            let project_config_dir = format!(
                "{}/projects/{}-{:x}",
                base_config_dir, project_name, project_hash
            );

            // Create project config directory if it doesn't exist
            if let Err(e) = fs::create_dir_all(&project_config_dir) {
                eprintln!("Warning: Failed to create project config directory: {}", e);
                eprintln!("Falling back to global config directory");
                // Fall back to global config
                let global_config_dir = format!("{}/global", base_config_dir);
                let _ = fs::create_dir_all(&global_config_dir);
                format!("{}/{}", global_config_dir, config_url)
            } else {
                format!("{}/{}", project_config_dir, config_url)
            }
        }
        Err(_) => {
            // Not in a git repository, use global config
            let global_config_dir = format!("{}/global", base_config_dir);
            let _ = fs::create_dir_all(&global_config_dir);
            format!("{}/{}", global_config_dir, config_url)
        }
    }
}

fn handle_config_subcommand(sub_matches: &clap::ArgMatches) {
    let config_dir = get_config_dir("config.toml");
    let config_dir_path = Path::new(&config_dir).parent().unwrap();

    // Initialize config if needed
    if let Err(e) = config::ensure_config_initialized(config_dir_path) {
        eprintln!("Error: Failed to initialize config: {}", e);
        std::process::exit(1);
    }

    match sub_matches.subcommand() {
        Some(("set", matches)) => {
            let key = matches
                .get_one::<String>("KEY")
                .expect("Required KEY missing");
            let value = matches
                .get_one::<String>("VALUE")
                .expect("Required VALUE missing");
            config::set_config_key(key, value, &config_dir).expect("Failed to set configuration");
        }
        Some(("get", matches)) => {
            let key = matches
                .get_one::<String>("KEY")
                .expect("Required KEY missing");
            let value = config::get_config_key(&[key.as_str()], &config_dir)
                .expect("Failed to get configuration")
                .join("\n");
            println!("{}", value);
        }
        Some(("list", _)) => {
            let keys = [
                "openai_api_key",
                "openai_url",
                "openai_model",
                "user_language",
            ];
            let values =
                config::get_config_key(&keys, &config_dir).expect("Failed to list configurations");
            for (key, value) in keys.iter().zip(values.iter()) {
                println!("{}: {}", key, value);
            }
        }
        _none => {
            eprintln!("Error: No subcommand provided for config");
            std::process::exit(1);
        }
    }
}
