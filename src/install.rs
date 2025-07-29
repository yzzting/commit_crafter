use std::collections::hash_map::DefaultHasher;
use std::env;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{self, Error, ErrorKind, Result, Write};
use std::path::{Path, PathBuf};

use crate::config::ensure_config_initialized;
use crate::git_integration;

fn get_project_config_dir() -> io::Result<PathBuf> {
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
            Ok(PathBuf::from(project_config_dir))
        }
        Err(e) => Err(Error::new(
            ErrorKind::NotFound,
            format!("Failed to get git root: {}", e),
        )),
    }
}

pub fn install_commit_msg_hook() -> Result<()> {
    let git_dir = Path::new(".git");
    if !git_dir.exists() || !git_dir.is_dir() {
        eprintln!("Error: Not a git repository");
        return Err(Error::new(ErrorKind::NotFound, "Not a git repository"));
    }

    // Try to get project-specific config directory, fallback to global if failed
    let config_dir = match get_project_config_dir() {
        Ok(dir) => {
            println!("Using project-specific config directory: {}", dir.display());
            dir
        }
        Err(e) => {
            eprintln!("Warning: Failed to get project config directory: {}", e);
            eprintln!("Falling back to global config directory");
            let home_dir = env::var("HOME").expect("Error getting home directory");
            let global_config_dir = format!("{}/.config/commit_crafter/global", home_dir);
            PathBuf::from(global_config_dir)
        }
    };

    // Initialize config
    if let Err(e) = ensure_config_initialized(&config_dir) {
        eprintln!("Error: Failed to initialize config: {}", e);
        return Err(Error::new(
            ErrorKind::Other,
            format!("Config initialization failed: {}", e),
        ));
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

    println!("Installed prepare-commit-msg hook successfully!");
    println!("Config directory: {}", config_dir.display());

    Ok(())
}
