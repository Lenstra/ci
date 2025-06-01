use crate::Error;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Rust {}

impl crate::runner::Runner for Rust {
    fn sniff(&self) -> Result<Vec<PathBuf>, Error> {
        crate::runner::path::dir(crate::runner::path::find(&["Cargo.toml"]))
    }

    fn format(&self, path: &PathBuf) -> Result<Vec<crate::runner::tasks::TaskGroup>, Error> {
        let mut task_group = crate::runner::tasks::TaskGroup::new(vec![]);
        task_group.append(crate::runner::tasks::Task::new(
            "cargo",
            vec!["fmt".to_string()],
            path.to_str().unwrap(),
        ));
        Ok(vec![task_group])
    }

    fn lint(&self, path: &PathBuf) -> Result<Vec<super::tasks::TaskGroup>, Error> {
        let mut task_group = crate::runner::tasks::TaskGroup::new(vec![]);
        task_group.append(crate::runner::tasks::Task::new(
            "cargo",
            vec!["check".to_string()],
            path.to_str().unwrap(),
        ));
        Ok(vec![task_group])
    }

    fn test(&self, path: &PathBuf) -> Result<Vec<super::tasks::TaskGroup>, Error> {
        let mut task_group = crate::runner::tasks::TaskGroup::new(vec![]);
        task_group.append(crate::runner::tasks::Task::new(
            "cargo",
            vec!["check".to_string()],
            path.to_str().unwrap(),
        ));
        Ok(vec![task_group])
    }
}
