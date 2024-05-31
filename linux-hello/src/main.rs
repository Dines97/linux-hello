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

    std::thread::scope(|s| {
        s.spawn(|| {
            log::info!("Lazy state loading");
            drop(crate::data::GLOBAL_DATA.read().unwrap());
            log::info!("Lazy state loaded");
        });
        s.spawn(|| {
            log::info!("Lazy config loading");
            drop(crate::config::GLOBAL_CONFIG.read().unwrap());
            log::info!("Lazy config loaded");
        });
        let cli = cli::Cli::parse();
        let _ = cli.command.run();
    });

    log::info!("Bye");

    Ok(())
}
