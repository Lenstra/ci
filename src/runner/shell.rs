use crate::Error;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Shell {}

impl crate::runner::Runner for Shell {
    fn sniff(&self) -> Result<Vec<PathBuf>, Error> {
        crate::runner::path::find(&["*.sh", "*.bash", ".bashrc"])
    }

    fn format(&self, path: &PathBuf) -> Result<Vec<crate::runner::tasks::TaskGroup>, Error> {
        Ok(vec![crate::runner::tasks::TaskGroup::single(
            crate::runner::tasks::Task::new(
                "shfmt",
                vec!["-w".to_string(), path.to_str().unwrap().to_string()],
                ".",
            ),
        )])
    }

    fn lint(&self, path: &PathBuf) -> Result<Vec<crate::runner::tasks::TaskGroup>, Error> {
        let mut task_group = crate::runner::tasks::TaskGroup::new(vec![]);
        task_group.append(crate::runner::tasks::Task::new(
            "shellcheck",
            vec![path.to_str().unwrap().to_string()],
            ".",
        ));
        Ok(vec![task_group])
    }

    fn test(&self, _path: &PathBuf) -> Result<Vec<super::tasks::TaskGroup>, Error> {
        Ok(vec![])
    }
}
