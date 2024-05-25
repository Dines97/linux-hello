use crate::config::GLOBAL_CONFIG;

use super::Runnable;
use clap::Args;
use color_eyre::Result;

#[derive(Debug, Args)]
pub(crate) struct ListArgs {}

impl Runnable for ListArgs {
    fn run(&self) -> Result<()> {
        let config = GLOBAL_CONFIG.read().unwrap();
        Ok(())
    }
}
