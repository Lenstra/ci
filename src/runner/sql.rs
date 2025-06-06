use crate::Error;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SQL {}

impl crate::runner::Runner for SQL {
    fn sniff(&self) -> Result<Vec<PathBuf>, Error> {
        Ok(vec![PathBuf::from(".")])
    }

    fn format(&self, path: &PathBuf) -> Result<Vec<crate::runner::tasks::TaskGroup>, Error> {
        Ok(vec![crate::runner::tasks::TaskGroup::single(
            crate::runner::tasks::Task::new(
                "sqlfmt",
                vec![".".to_string()],
                path.to_str().unwrap(),
            ),
        )])
    }

    fn lint(&self, _path: &PathBuf) -> Result<Vec<super::tasks::TaskGroup>, Error> {
        Ok(vec![])
    }

    fn test(&self, _path: &PathBuf) -> Result<Vec<super::tasks::TaskGroup>, Error> {
        Ok(vec![])
    }
}
