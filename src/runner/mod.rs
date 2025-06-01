use crate::Error;
use std::path::PathBuf;

pub mod go;
pub mod path;
pub mod python;
pub mod rust;
pub mod shell;
pub mod sql;
pub mod tasks;

pub trait Runner {
    fn sniff(&self) -> Result<Vec<PathBuf>, Error>;
    fn format(&self, path: &PathBuf) -> Result<Vec<tasks::TaskGroup>, Error>;
    fn lint(&self, path: &PathBuf) -> Result<Vec<tasks::TaskGroup>, Error>;
    fn test(&self, path: &PathBuf) -> Result<Vec<tasks::TaskGroup>, Error>;
}
