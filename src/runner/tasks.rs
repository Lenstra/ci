use crate::ui;
use std::error::Error;

/// A single task that will run in parallel with other tasks.
#[derive(Debug, Clone)]
pub struct Task {
    pub command: String,
    pub args: Vec<String>,
    pub workdir: String,
}

impl Task {
    pub fn new(command: &str, args: Vec<String>, workdir: &str) -> Self {
        Self {
            command: command.to_string(),
            args,
            workdir: workdir.to_string(),
        }
    }
}

/// A group of tasks that need to run sequentially.
#[derive(Clone)]
pub struct TaskGroup {
    pub runner: String,
    pub tasks: Vec<Task>,
}

impl TaskGroup {
    pub fn new(tasks: Vec<Task>) -> Self {
        Self {
            runner: String::new(),
            tasks,
        }
    }

    pub fn single(task: Task) -> Self {
        Self {
            runner: String::new(),
            tasks: vec![task],
        }
    }

    pub fn append(&mut self, task: Task) {
        self.tasks.push(task);
    }

    pub fn run(
        &self,
        ui: &ui::UI,
        runner_name: &str,
        dry_run: bool,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        for task in &self.tasks {
            ui.debug(&format!(
                "{}",
                format!("[{}] Running {:?}", runner_name, task)
            ));
            if !dry_run {
                crate::command::run(&task.command, task.args.clone(), &task.workdir)?;
            }
            ui.trace(&format!(
                "{}",
                format!("[{}] Task {:?} completed", runner_name, task)
            ));
        }
        Ok(())
    }
}
