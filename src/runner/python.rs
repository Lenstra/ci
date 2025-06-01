use crate::Error;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Python {}

impl crate::runner::Runner for Python {
    fn sniff(&self) -> Result<Vec<PathBuf>, Error> {
        Ok(vec![PathBuf::from(".")])
    }

    fn format(&self, path: &PathBuf) -> Result<Vec<crate::runner::tasks::TaskGroup>, Error> {
        Ok(vec![crate::runner::tasks::TaskGroup::new(vec![
            crate::runner::tasks::Task::new(
                "isort",
                vec![
                    "--profile".to_string(),
                    "black".to_string(),
                    "--no-sections".to_string(),
                    "--lines-between-types".to_string(),
                    "1".to_string(),
                    ".".to_string(),
                ],
                path.to_str().unwrap(),
            ),
            crate::runner::tasks::Task::new("black", vec![".".to_string()], path.to_str().unwrap()),
        ])])
    }

    fn lint(&self, _path: &PathBuf) -> Result<Vec<super::tasks::TaskGroup>, Error> {
        Ok(vec![])
    }

    fn test(&self, _path: &PathBuf) -> Result<Vec<super::tasks::TaskGroup>, Error> {
        Ok(vec![])
    }
}
