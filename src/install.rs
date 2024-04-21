use std::env;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

pub fn install_commit_msg_hook() {
    print!("This will install the pre-commit hook. Continue? (y/n): ");
    let git_dir = Path::new(".git");
    if !git_dir.exists() || !git_dir.is_dir() {
        eprintln!("Error: Not a git repository");
        std::process::exit(1);
    }

    let current_exe_path = env::current_exe().expect("Error getting current executable path");
    let hooks_dir = git_dir.join("hooks");
    let pre_commit_hook = hooks_dir.join("pre-commit");

    if pre_commit_hook.exists() {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Error reading input");
        if input.trim() != "y" {
            eprintln!("Error: pre-commit hook already exists. Exiting...");
            std::process::exit(1);
        }
    }

    let mut file = File::create(pre_commit_hook).expect("Error creating pre-commit hook");
    writeln!(
        file,
        "#!/bin/sh\n\n{} \"$@\"\n\nexit 0",
        current_exe_path.to_string_lossy()
    )
    .expect("Error writing to pre-commit hook file");

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
