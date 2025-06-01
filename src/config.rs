use crate::runner::{go, python, rust, shell, sql};

use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub sql: Option<sql::SQL>,
    pub go: Option<go::Go>,
    pub python: Option<python::Python>,
    pub rust: Option<rust::Rust>,
    pub shell: Option<shell::Shell>,
}

impl Config {
    fn get_config_path(
        path: Option<String>,
    ) -> Result<std::path::PathBuf, Box<dyn Error + Send + Sync>> {
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

    pub fn from_file(
        path: Option<String>,
    ) -> Result<(std::path::PathBuf, Self), Box<dyn Error + Send + Sync>> {
        let path = Self::get_config_path(path)?;
        let dir = path.parent().unwrap().to_path_buf();
        let contents = std::fs::read_to_string(&path)?;
        let config: Config = hcl::from_str(&contents)?;
        Ok((dir, config))
    }
}
