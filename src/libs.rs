//! This lib wraps svn command line tool on your system
#![warn(missing_docs)]
#![warn(unsafe_code)]
use std::process::Command;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SvnError {
    #[error("Failed to run svn command: {0}")]
    CommandFailed(String),
}

/// Returns the version of svn command line tool
pub fn version() -> String {
    let output = Command::new("svn")
        .arg("--version")
        .output()
        .expect("Failed to execute command");

    String::from_utf8_lossy(&output.stdout).into_owned()
}

/// This struct wraps svn command line tool
pub struct SvnWrapper {}

impl SvnWrapper {
    pub fn new() -> Self {
        Self {}
    }

    pub fn commit(&self, path: &str) -> Result<(), SvnError> {
        let output = Command::new("svn")
            .arg("commit")
            .arg("-m")
            .arg("\"Committed changes\"")
            .arg(path)
            .output()
            .map_err(|e| SvnError::CommandFailed(e.to_string()))?;

        if !output.status.success() {
            return Err(SvnError::CommandFailed(
                String::from_utf8_lossy(&output.stderr).to_string(),
            ));
        }

        Ok(())
    }

    pub fn checkout(&self, url: &str, path: &str) -> Result<(), SvnError> {
        let output = Command::new("svn")
            .arg("checkout")
            .arg(url)
            .arg(path)
            .output()
            .map_err(|e| SvnError::CommandFailed(e.to_string()))?;

        if !output.status.success() {
            return Err(SvnError::CommandFailed(
                String::from_utf8_lossy(&output.stderr).to_string(),
            ));
        }

        Ok(())
    }

    pub fn update(&self, path: &str) -> Result<(), SvnError> {
        let output = Command::new("svn")
            .arg("update")
            .arg(path)
            .output()
            .map_err(|e| SvnError::CommandFailed(e.to_string()))?;

        if !output.status.success() {
            return Err(SvnError::CommandFailed(
                String::from_utf8_lossy(&output.stderr).to_string(),
            ));
        }

        Ok(())
    }

    pub fn log(&self, path: &str) -> Result<String, SvnError> {
        let output = Command::new("svn")
            .arg("log")
            .arg(path)
            .output()
            .map_err(|e| SvnError::CommandFailed(e.to_string()))?;

        if !output.status.success() {
            return Err(SvnError::CommandFailed(
                String::from_utf8_lossy(&output.stderr).to_string(),
            ));
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}

pub struct SvnInfo {
    pub url: String,
    pub repository_root: String,
    pub last_changed_author: String,
    pub last_changed_rev: u32,
    pub last_changed_date: String,
}

impl SvnInfo {
    pub fn new(path: &str) -> Result<SvnInfo, SvnError> {
        let output = Command::new("svn")
            .arg("info")
            .arg(path)
            .output()
            .map_err(|e| SvnError::CommandFailed(e.to_string()))?;

        if !output.status.success() {
            return Err(SvnError::CommandFailed(
                String::from_utf8_lossy(&output.stderr).to_string(),
            ));
        }

        let output_str = String::from_utf8_lossy(&output.stdout).to_string();

        let url = output_str.lines().find(|line| line.starts_with("URL: ")).map(|line| line[5..].to_owned());
        let repository_root = output_str.lines().find(|line| line.starts_with("Repository Root: ")).map(|line| line[17..].to_owned());
        let last_changed_author = output_str.lines().find(|line| line.starts_with("Last Changed Author: ")).map(|line| line[22..].to_owned());
        let last_changed_rev = output_str.lines().find(|line| line.starts_with("Last Changed Rev: ")).and_then(|line| line[19..].parse::<u32>().ok());
        let last_changed_date = output_str.lines().find(|line| line.starts_with("Last Changed Date: ")).map(|line| line[20..].to_owned());

        if let (Some(url), Some(repository_root), Some(last_changed_author), Some(last_changed_rev), Some(last_changed_date)) = (url, repository_root, last_changed_author, last_changed_rev, last_changed_date) {
            Ok(SvnInfo {
                url,
                repository_root,
                last_changed_author,
                last_changed_rev,
                last_changed_date,
            })
        } else {
            Err(SvnError::CommandFailed("Unable to parse svn info output".to_owned()))
        }
    }
}

pub struct SvnStatus {
    pub item: String,
    pub status: String,
    pub repository_status: String,
    pub working_copy_status: String,
}

impl SvnStatus {
    pub fn new(path: &str) -> Result<Vec<SvnStatus>, SvnError> {
        let output = Command::new("svn")
            .arg("status")
            .arg("--show-updates")
            .arg(path)
            .output()
            .map_err(|e| SvnError::CommandFailed(e.to_string()))?;

        if !output.status.success() {
            return Err(SvnError::CommandFailed(
                String::from_utf8_lossy(&output.stderr).to_string(),
            ));
        }

        let output_str = String::from_utf8_lossy(&output.stdout).to_string();
        let mut statuses = Vec::new();

        for line in output_str.lines() {
            let parts: Vec<&str> = line.split(' ').collect();

            if parts.len() < 2 {
                continue;
            }

            let status = parts[0];
            let item = parts[1];
            let repository_status = parts.get(2).map(|s| s.to_owned()).unwrap_or_default();
            let working_copy_status = parts.get(3).map(|s| s.to_owned()).unwrap_or_default();

            statuses.push(SvnStatus {
                item: item.to_owned(),
                status: status.to_owned(),
                repository_status: repository_status.to_owned(),
               working_copy_status: working_copy_status.to_owned(),
            });
        }

        Ok(statuses)
    }
}
