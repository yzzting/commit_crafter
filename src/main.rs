extern crate commit_crafter;

use commit_crafter::{config, git_integration, install, llm};

use clap::{arg, Command};

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
        .subcommand(
            Command::new("config")
                .about("Configure settings")
                .version("0.1.0")
                .author("ZenYang <yzzting@gmail.com>")
                .subcommand(
                    Command::new("set")
                        .about("Set a configuration option")
                        .arg(arg!(<KEY> "The configuration key to set"))
                        .arg(arg!(<VALUE> "The value to set the key to")),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("install", _sub_matches)) => {
            let _ = install::install_commit_msg_hook();
        }
        Some(("config", sub_matches)) => {
            if let Some(set_matches) = sub_matches.subcommand_matches("set") {
                let key = set_matches.get_one::<String>("KEY").unwrap();
                let value = set_matches.get_one::<String>("VALUE").unwrap();
                println!("Setting {} to {}", key, value);
                config::set_config_key(key, value, "config.toml").unwrap();
            } else {
                eprintln!("Error: No value provided for --set");
                std::process::exit(1);
            }
        }
        None => match git_integration::run_git_diff() {
            Ok(output) => {
                if output.is_empty() {
                    eprintln!("Error: No changes to commit");
                    std::process::exit(1);
                }
                llm::openai::openai_request(&output, "config.toml").unwrap();
            }
            Err(e) => eprintln!("Error: {}", e),
        },
        _ => (),
    }
}
