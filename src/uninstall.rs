use std::fs;
use std::path::Path;
use std::io::{Error, ErrorKind, Result};

pub fn uninstall_commit_msg_hook() -> Result<()> {
    let git_dir = Path::new(".git");
    if !git_dir.exists() || !git_dir.is_dir() {
        eprintln!("Error: Not a git repository");
        return Err(Error::new(ErrorKind::NotFound, "Not a git repository"));
    }

    let hooks_dir = git_dir.join("hooks");
    let pre_commit_hook = hooks_dir.join("prepare-commit-msg");

    if !pre_commit_hook.exists() {
        eprintln!("Error: prepare-commit-msg hook does not exist");
        return Err(Error::new(
            ErrorKind::NotFound,
            "prepare-commit-msg hook does not exist",
        ));
    }

    fs::remove_file(pre_commit_hook).expect("Error removing prepare-commit-msg hook");
    println!("Uninstalled prepare-commit-msg hook");
    Ok(())
}
