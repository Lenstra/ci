mod config;

use clap::Parser;
use runner::{Runner, tasks::TaskGroup};

pub mod command;
pub mod runner;
pub mod ui;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    config: Option<String>,

    #[arg(long)]
    runner: Option<String>,

    #[arg(long, default_value = "info")]
    log_level: String,

    #[arg(long)]
    dry_run: bool,
}

fn main() {
    let args = Args::parse();
    let ui = ui::UI::new(args.log_level.clone().into());
    if let Err(e) = run(&ui, args) {
        ui.error(&format!("Error: {}", e));
        std::process::exit(1);
    }
}

pub type Error = Box<dyn std::error::Error + Send + Sync>;

fn run(ui: &ui::UI, args: Args) -> Result<(), Error> {
    let (dir, config) = config::Config::from_file(args.config)?;
    std::env::set_current_dir(dir)?;

    let mut runners = Vec::new();
    for runner in [
        (
            "sql",
            config.sql.clone().map(|r| Box::new(r) as Box<dyn Runner>),
        ),
        (
            "go",
            config.go.clone().map(|r| Box::new(r) as Box<dyn Runner>),
        ),
        (
            "python",
            config
                .python
                .clone()
                .map(|r| Box::new(r) as Box<dyn Runner>),
        ),
        (
            "rust",
            config.rust.clone().map(|r| Box::new(r) as Box<dyn Runner>),
        ),
        (
            "shell",
            config.shell.clone().map(|r| Box::new(r) as Box<dyn Runner>),
        ),
    ] {
        if let (runner_name, Some(r)) = runner {
            if let Some(ref filter_name) = args.runner {
                if runner_name != filter_name {
                    continue;
                }
            }

            runners.push((runner_name.to_string(), r));
        }
    }

    if runners.is_empty() {
        return Err("No runner found".into());
    }

    let mut task_groups: Vec<TaskGroup> = Vec::new();
    for (runner_name, runner) in runners {
        for path in runner.sniff()? {
            for task_group in runner
                .format(&path)?
                .iter()
                .chain(runner.lint(&path)?.iter())
            {
                let mut task_group = task_group.clone();
                task_group.runner = runner_name.clone();
                task_groups.push(task_group);
            }
        }
    }

    if task_groups.is_empty() {
        ui.error(&format!(
            "No runner found matching {}",
            args.runner.unwrap()
        ));
        std::process::exit(1);
    }

    run_task_groups(&ui, task_groups, args.dry_run)?;

    Ok(())
}

fn run_task_groups(ui: &ui::UI, task_groups: Vec<TaskGroup>, dry_run: bool) -> Result<(), Error> {
    let handles: Vec<_> = task_groups
        .into_iter()
        .map(|task_group| {
            let runner_name = task_group.runner.clone();
            let ui = ui.clone();
            let dry_run = dry_run.clone();
            std::thread::spawn(move || task_group.run(&ui, &runner_name, dry_run))
        })
        .collect();

    for handle in handles {
        handle.join().unwrap()?;
    }
    Ok(())
}
