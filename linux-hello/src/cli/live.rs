use crate::{
    core::{Core, OperationMode},
    Runnable,
};
use clap::Args;
use color_eyre::Result;

#[derive(Debug, Args)]
pub(crate) struct LiveArgs {
    #[arg(short, long)]
    pub camera: Option<i32>,
}

impl Runnable for LiveArgs {
    fn run(&self) -> Result<()> {
        let mut core = Core::new(self.camera, OperationMode::Live)?;
        core.login();

        Ok(())
    }
}
