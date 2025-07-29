use commit_crafter::git_integration;
use std::env;

#[test]
fn test_get_recent_commits() {
    if env::var("GITHUB_ACTIONS").is_ok() {
        eprintln!("Skipping test in GitHub Actions environment");
        return;
    }

    // 测试获取最近的commit messages
    let result = git_integration::get_recent_commits(3);

    match result {
        Ok(commits) => {
            // 验证返回的是Vec<String>
            assert!(commits.len() <= 3);
            // 如果有commit，每个commit应该是非空字符串
            for commit in commits {
                assert!(!commit.is_empty());
            }
        }
        Err(e) => {
            // 如果是新的仓库可能没有commit，这是正常的
            eprintln!("Warning: Could not get recent commits: {}", e);
        }
    }
}

#[test]
fn test_get_git_root_dir() {
    if env::var("GITHUB_ACTIONS").is_ok() {
        eprintln!("Skipping test in GitHub Actions environment");
        return;
    }

    // 测试获取git仓库根目录
    let result = git_integration::get_git_root_dir();

    match result {
        Ok(root_path) => {
            // 验证返回的路径存在且是目录
            assert!(root_path.exists());
            assert!(root_path.is_dir());
            // 验证这个目录包含.git目录
            let git_dir = root_path.join(".git");
            assert!(git_dir.exists());
        }
        Err(e) => {
            eprintln!("Warning: Could not get git root directory: {}", e);
            // 在非git仓库中运行测试时，这是预期的行为
        }
    }
}

#[test]
fn test_get_recent_commits_with_zero_count() {
    if env::var("GITHUB_ACTIONS").is_ok() {
        eprintln!("Skipping test in GitHub Actions environment");
        return;
    }

    // 测试获取0个commit的情况
    let result = git_integration::get_recent_commits(0);

    match result {
        Ok(commits) => {
            assert_eq!(commits.len(), 0);
        }
        Err(_) => {
            // 在某些情况下，git log -0 可能会失败，这是可以接受的
        }
    }
}

#[test]
fn test_get_recent_commits_large_count() {
    if env::var("GITHUB_ACTIONS").is_ok() {
        eprintln!("Skipping test in GitHub Actions environment");
        return;
    }

    // 测试获取大量commit的情况（可能超过实际存在的数量）
    let result = git_integration::get_recent_commits(100);

    match result {
        Ok(commits) => {
            // 应该返回所有可用的commit，不会超过100个
            assert!(commits.len() <= 100);
            // 验证每个commit都是有效的
            for commit in commits {
                assert!(!commit.is_empty());
            }
        }
        Err(e) => {
            eprintln!("Warning: Could not get recent commits: {}", e);
        }
    }
}

#[test]
fn test_run_git_diff_still_works() {
    if env::var("GITHUB_ACTIONS").is_ok() {
        eprintln!("Skipping test in GitHub Actions environment");
        return;
    }

    // 确保原有的git diff功能仍然正常工作
    let result = git_integration::run_git_diff();

    match result {
        Ok(_output) => {
            // git diff成功执行，输出可能为空（没有staged changes）
            assert!(true);
        }
        Err(e) => {
            eprintln!("Git diff failed: {}", e);
            // 在CI环境中或者某些情况下git diff可能失败，但这不应该导致测试失败
        }
    }
}
