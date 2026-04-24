mod add;
mod bulk_add;
mod bulk_remove;
mod list;
mod remove;

#[derive(Debug, clap::Subcommand)]
#[command(about = "tools for managing GitHub Enterprise Teams Organization assignments")]
pub enum Command {
  Add(add::Command),
  Remove(remove::Command),
  List(list::Command),
  BulkAdd(bulk_add::Command),
  BulkRemove(bulk_remove::Command),
}

impl Command {
  pub async fn run(self) -> Result<(), crate::Error> {
    match self {
      Self::Add(command) => command.run().await,
      Self::Remove(command) => command.run().await,
      Self::List(command) => command.run().await,
      Self::BulkAdd(command) => command.run().await,
      Self::BulkRemove(command) => command.run().await,
    }
  }
}
