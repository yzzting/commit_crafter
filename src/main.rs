extern crate commit_crafter;

use commit_crafter::{config, git_integration, install, llm, uninstall};

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
                .version("0.1.0")
                .author("ZenYang <yzzting@gmail.com"),
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

fn handle_config_subcommand(sub_matches: &clap::ArgMatches) {
    match sub_matches.subcommand() {
        Some(("set", matches)) => {
            let key = matches
                .get_one::<String>("KEY")
                .expect("Required KEY missing");
            let value = matches
                .get_one::<String>("VALUE")
                .expect("Required VALUE missing");
            config::set_config_key(key, value, "config.toml").expect("Failed to set configuration");
        }
        Some(("get", matches)) => {
            let key = matches
                .get_one::<String>("KEY")
                .expect("Required KEY missing");
            let value = config::get_config_key(&[key.as_str()], "config.toml")
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
            let values = config::get_config_key(&keys, "config.toml")
                .expect("Failed to list configurations");
            for (key, value) in keys.iter().zip(values.iter()) {
                println!("{}: {}", key, value);
            }
        }
        None => {
            eprintln!("Error: No subcommand provided for config");
            std::process::exit(1);
        }
        _ => (),
    }
}
