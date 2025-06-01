mod config;

use clap::Parser;

pub mod runner;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    config: Option<String>,
}

fn main() {
    let args = Args::parse();
    if let Err(e) = run(args) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    let config = config::Config::from_file(args.config)?;

    if let Some(sql) = config.sql {
        sql.format()?;
    }

    Ok(())
}
