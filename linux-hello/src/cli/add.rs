use super::Runnable;
use crate::core::{Core, OperationMode};
use clap::Args;
use color_eyre::Result;

#[derive(Debug, Args)]
pub(crate) struct AddArgs {
    #[arg(short, long)]
    pub camera: Option<i32>,
}

impl Runnable for AddArgs {
    fn run(&self) -> Result<()> {
        let mut core = Core::new(self.camera, OperationMode::Add);
        core.add();

        Ok(())
    }
}
