use super::Runnable;
use crate::core::{Core, OperationMode};
use clap::Args;
use color_eyre::Result;

#[derive(Debug, Args)]
pub(crate) struct AddArgs {}

impl Runnable for AddArgs {
    fn run(&self) -> Result<()> {
        Core::new(Default::default(), OperationMode::Add);

        Ok(())
    }
}
