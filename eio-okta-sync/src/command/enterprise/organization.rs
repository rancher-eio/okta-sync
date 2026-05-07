mod list;

#[derive(Debug, clap::Subcommand)]
#[command(about = "tools for managing GitHub Enterprise Organizations")]
pub enum Command {
  List(list::Command),
}

impl Command {
  pub async fn run(self) -> Result<(), crate::Error> {
    match self {
      Self::List(command) => command.run().await,
    }
  }
}
