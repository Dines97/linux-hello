mod cli;
mod config;
mod core;
mod data;

use clap::Parser;
use cli::Runnable;
use env_logger::{Builder, Target};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    Builder::from_default_env()
        .target(Target::Stdout)
        .filter_level(log::LevelFilter::Trace)
        .init();

    let cli = cli::Cli::parse();
    let _ = cli.command.run();

    log::info!("Bye");

    Ok(())
}

