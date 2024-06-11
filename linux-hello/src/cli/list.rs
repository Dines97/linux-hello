use super::Runnable;
use clap::Args;
use color_eyre::Result;
use comfy_table::Table;

#[derive(Debug, Args)]
pub(crate) struct ListArgs {}

impl Runnable for ListArgs {
    fn run(&self) -> Result<()> {
        let data = crate::data::read();

        let mut table = Table::new();
        table.load_preset(comfy_table::presets::NOTHING);
        let mut id = 0;

        data.users.iter().flat_map(|x| x.identities.iter()).for_each(|x| {
            table.add_row(vec![format!("{id}"), x.name.clone()]);
            id += 1;
        });

        println!("{table}");
        Ok(())
    }
}
