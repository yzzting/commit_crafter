use commit_crafter::uninstall;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_uninstall_commit_msg_hook() {
    let temp_dir = tempdir().unwrap();
    let git_dir = temp_dir.path().join(".git");
    let hooks_dir = git_dir.join("hooks");

    // create config.toml
    fs::write(temp_dir.path().join("config.toml"), "").unwrap();

    // mock git Environment
    fs::create_dir_all(&hooks_dir).unwrap();
    fs::write(hooks_dir.join("prepare-commit-msg"), "").unwrap();

    // set the environment to use the temp directory
    std::env::set_current_dir(temp_dir.path()).unwrap();

    let _ = uninstall::uninstall_commit_msg_hook();

    // check if the prepare-commit-msg hook was removed
    assert!(!hooks_dir.join("prepare-commit-msg").exists());

    // clean up
    temp_dir.close().unwrap();
}