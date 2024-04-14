mod add;
mod download;
mod live;

use ambassador::{delegatable_trait, Delegate};
use clap::{Parser, Subcommand};
use color_eyre::Result;
use shadow_rs::shadow;

shadow!(build);

#[delegatable_trait]
pub(crate) trait Runnable {
    fn run(&self) -> Result<()>;
}

#[derive(Parser, Debug)]
#[command(version = build::CLAP_LONG_VERSION)]
#[command(about, long_about = None)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) command: Commands,
}

#[derive(Subcommand, Debug, Delegate)]
#[delegate(Runnable)]
pub(crate) enum Commands {
    Add(add::AddArgs),
    Live(live::LiveArgs),
    Download(download::DownloadArgs),
}
