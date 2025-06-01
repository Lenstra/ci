use crate::Error;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Go {}

impl crate::runner::Runner for Go {
    fn sniff(&self) -> Result<Vec<PathBuf>, Error> {
        crate::runner::path::dir(crate::runner::path::find(&["go.mod"]))
    }

    fn format(&self, path: &PathBuf) -> Result<Vec<crate::runner::tasks::TaskGroup>, Error> {
        Ok(vec![crate::runner::tasks::TaskGroup::new(vec![
            crate::runner::tasks::Task::new(
                "go",
                vec!["mod".to_string(), "tidy".to_string()],
                path.to_str().unwrap(),
            ),
            crate::runner::tasks::Task::new(
                "go",
                vec!["fmt".to_string(), "./...".to_string()],
                path.to_str().unwrap(),
            ),
        ])])
    }

    fn lint(&self, _path: &PathBuf) -> Result<Vec<super::tasks::TaskGroup>, Error> {
        Ok(vec![])
    }

    fn test(&self, _path: &PathBuf) -> Result<Vec<super::tasks::TaskGroup>, Error> {
        Ok(vec![])
    }
}
