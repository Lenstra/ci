use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SQL {}

impl SQL {
    pub fn format(&self) -> Result<(), Box<dyn std::error::Error>> {
        let output = std::process::Command::new("sqlfmt")
            .arg(".")
            .output()?;

        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).into());
        }

        Ok(())
    }
}
