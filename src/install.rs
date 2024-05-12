use std::env;
use std::fs::{self, File};
use std::io::{self, Error, ErrorKind, Result, Write};
use std::path::Path;

use crate::config::{generate_config_toml, move_prompt_toml, write_config_to_toml};

pub fn install_commit_msg_hook() -> Result<()> {
    let git_dir = Path::new(".git");
    if !git_dir.exists() || !git_dir.is_dir() {
        eprintln!("Error: Not a git repository");
        return Err(Error::new(ErrorKind::NotFound, "Not a git repository"));
    }

    let home_dir = env::var("HOME").expect("Error getting home directory");
    let config_dir = Path::new(&home_dir).join(".config/commit_crafter");
    let config_dif_clone = config_dir.clone();
    fs::create_dir_all(config_dif_clone).expect("Error creating config directory");
    let config_toml = config_dir.join("config.toml");
    if !config_toml.exists() {
        println!("Error: config.toml not found. Start generating default files.");
        let config_str = generate_config_toml();
        if let Err(e) = write_config_to_toml(&config_str, &config_toml) {
            eprintln!("Failed to write default config.toml: {}", e);
            return Err(e);
        }
    }

    let prompt_toml = config_dir.join("prompt.toml");
    if !prompt_toml.exists() {
        println!("Warn: prompt.toml not found. Start generating default files.");
        move_prompt_toml(&prompt_toml);
    }

    let current_exe_path = env::current_exe()
        .expect("Error getting current executable path")
        .to_string_lossy()
        .to_string();
    let hooks_dir = git_dir.join("hooks");
    let pre_commit_hook = hooks_dir.join("prepare-commit-msg");

    if pre_commit_hook.exists() {
        println!("pre_commit_hook is exists, do you want to overwrite it? [y/n]");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Error reading input");
        if input.trim() != "y" {
            eprintln!("Error: prepare-commit-msg hook already exists. Exiting...");
            return Err(Error::new(
                ErrorKind::AlreadyExists,
                "prepare-commit-msg hook already exists",
            ));
        }
    }

    let mut file = File::create(pre_commit_hook).expect("Error creating prepare-commit-msg hook");
    let shell_script = format!(
        r#"
#!/bin/sh

echo "Generating commit message from OpenAI..."
COMMIT_MSG=$({})
RETVAL=$?
if [ $RETVAL -ne 0 ]; then
    echo "Failed to generate commit message!!"
    exit 1
fi
# Check if the commit message is non-empty
if [ -z "$COMMIT_MSG" ]; then
    echo "Empty commit message generated. Commit aborted."
    exit 1
fi
echo "$COMMIT_MSG" > $1
        "#,
        current_exe_path
    );
    writeln!(file, "{}", shell_script).expect("Error writing to prepare-commit-msg hook file");

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut permissions = file
            .metadata()
            .expect("Error getting file metadata")
            .permissions();
        permissions.set_mode(0o755);
        file.set_permissions(permissions)
            .expect("Error setting file permissions");
    }

    Ok(())
}
