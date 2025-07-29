use std::io::{self, ErrorKind};
use std::path::PathBuf;
use std::process::Command;

pub fn exclude_from_diff(path: &str) -> String {
    format!(":(exclude){}", path)
}

pub fn run_git_diff() -> Result<String, io::Error> {
    let files_to_exclude = vec![
        "Cargo.lock",
        "pakcage-lock.json",
        "pnpm-lock.yaml",
        "*.lock",
    ];

    let exclude_path: Vec<String> = files_to_exclude
        .iter()
        .map(|path| exclude_from_diff(path))
        .collect();

    let mut command = Command::new("git");
    command.args(&[
        "diff",
        "--staged",
        "--ignore-all-space",
        "--diff-algorithm=minimal",
    ]);

    for path in exclude_path {
        command.arg(path);
    }

    let output = command.output();

    match output {
        Ok(output) if output.status.success() => match String::from_utf8(output.stdout) {
            Ok(output_str) => Ok(output_str),
            Err(e) => Err(io::Error::new(
                ErrorKind::InvalidData,
                format!("Output is not valid UTF-8: {}", e),
            )),
        },
        Ok(output) => {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(io::Error::new(
                ErrorKind::Other,
                format!("Error: {}", stderr),
            ))
        }
        Err(e) => Err(e),
    }
}

pub fn get_recent_commits(count: usize) -> Result<Vec<String>, io::Error> {
    let command = Command::new("git")
        .args(&["log", &format!("-{}", count), "--pretty=format:%s"])
        .output();

    match command {
        Ok(output) if output.status.success() => match String::from_utf8(output.stdout) {
            Ok(output_str) => {
                let commits: Vec<String> = output_str
                    .lines()
                    .map(|line| line.trim().to_string())
                    .filter(|line| !line.is_empty())
                    .collect();
                Ok(commits)
            }
            Err(e) => Err(io::Error::new(
                ErrorKind::InvalidData,
                format!("Output is not valid UTF-8: {}", e),
            )),
        },
        Ok(output) => {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(io::Error::new(
                ErrorKind::Other,
                format!("Error getting recent commits: {}", stderr),
            ))
        }
        Err(e) => Err(e),
    }
}

pub fn get_git_root_dir() -> Result<PathBuf, io::Error> {
    let command = Command::new("git")
        .args(&["rev-parse", "--show-toplevel"])
        .output();

    match command {
        Ok(output) if output.status.success() => match String::from_utf8(output.stdout) {
            Ok(output_str) => {
                let root_path = output_str.trim();
                Ok(PathBuf::from(root_path))
            }
            Err(e) => Err(io::Error::new(
                ErrorKind::InvalidData,
                format!("Output is not valid UTF-8: {}", e),
            )),
        },
        Ok(output) => {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(io::Error::new(
                ErrorKind::Other,
                format!("Error getting git root directory: {}", stderr),
            ))
        }
        Err(e) => Err(e),
    }
}
