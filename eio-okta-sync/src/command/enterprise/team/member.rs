mod add;
mod bulk_add;
mod bulk_remove;
mod list;
mod remove;

#[derive(Debug, clap::Subcommand)]
#[command(about = "tools for managing enterprise team members")]
pub enum Command {
  Add(add::Command),
  BulkAdd(bulk_add::Command),
  BulkRemove(bulk_remove::Command),
  List(list::Command),
  Remove(remove::Command),
}

impl Command {
  pub async fn run(self) -> Result<(), crate::Error> {
    match self {
      Self::Add(command) => command.run().await,
      Self::BulkAdd(command) => command.run().await,
      Self::BulkRemove(command) => command.run().await,
      Self::List(command) => command.run().await,
      Self::Remove(command) => command.run().await,
    }
  }
}
