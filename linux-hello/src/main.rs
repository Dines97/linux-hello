mod cli;
mod config;
mod core;
mod cycle_controller;
mod state;

use clap::Parser;
use cli::Runnable;
use env_logger::{Builder, Target};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    Builder::from_default_env()
        .target(Target::Stdout)
        .filter_level(log::LevelFilter::Trace)
        .init();

    opencv_sys::set_num_threads(1);

    state::deserialize();

    let cli = cli::Cli::parse();
    let _ = cli.command.run();

    state::serialize();

    log::info!("Bye");

    Ok(())
}
