use commit_crafter::install;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_install_commit_msg_hook() {
    let temp_dir = tempdir().unwrap();
    let git_dir = temp_dir.path().join(".git");
    let hooks_dir = git_dir.join("hooks");

    // create config.toml
    fs::write(temp_dir.path().join("config.toml"), "").unwrap();

    // mock git Environment
    fs::create_dir_all(&hooks_dir).unwrap();

    // set the environment to use the temp directory
    std::env::set_current_dir(temp_dir.path()).unwrap();

    let _ = install::install_commit_msg_hook();

    // check if the prepare-commit-msg hook was created
    assert!(hooks_dir.join("prepare-commit-msg").exists());

    // clean up
    temp_dir.close().unwrap();
}

#[test]
fn test_install_commit_msg_hook_with_no_git_dir() {
    let temp_dir = tempdir().unwrap();

    // create config.toml
    fs::write(temp_dir.path().join("config.toml"), "").unwrap();

    // set the environment to use the temp directory
    std::env::set_current_dir(temp_dir.path()).unwrap();

    let result = install::install_commit_msg_hook();

    // check if is not a git repository
    assert!(result.is_err());

    // clean up
    temp_dir.close().unwrap();
}
