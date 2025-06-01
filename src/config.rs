use crate::runner::sql;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub sql: Option<sql::SQL>,
}

impl Config {
    fn get_config_path(path: Option<String>) -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
        if let Some(path) = path {
            return Ok(std::path::PathBuf::from(path));
        }

        let output = std::process::Command::new("git")
            .args(["rev-parse", "--show-toplevel"])
            .output()?;

        if !output.status.success() {
            return Err("Not in a git repository".into());
        }

        let repo_root = String::from_utf8(output.stdout)?.trim().to_string();
        let ci_path = std::path::Path::new(&repo_root).join("ci.hcl");

        if ci_path.exists() {
            Ok(ci_path)
        } else {
            Err(format!("{}: No ci.hcl found in git repository", repo_root).into())
        }
    }

    pub fn from_file(path: Option<String>) -> Result<Self, Box<dyn std::error::Error>> {
        let path = Self::get_config_path(path)?;
        let contents = std::fs::read_to_string(path)?;
        let config: Config = hcl::from_str(&contents)?;
        Ok(config)
    }
}
