use std::env;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

use crate::config::generate_config_toml;

pub fn install_commit_msg_hook() {
    let git_dir = Path::new(".git");
    if !git_dir.exists() || !git_dir.is_dir() {
        eprintln!("Error: Not a git repository");
        std::process::exit(1);
    }

    let config_toml = Path::new("config.toml");
    if !config_toml.exists() {
        println!("Error: config.toml not found. Start generating default files.");
        generate_config_toml();
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
            std::process::exit(1);
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
}
