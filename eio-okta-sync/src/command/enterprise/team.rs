mod assign_members;

#[derive(Debug, clap::Subcommand)]
#[command(about = "tools for managing GitHub Enterprise Teams")]
pub enum Command {
  AssignMembers(assign_members::Command),
}

impl Command {
  pub async fn run(self) -> Result<(), crate::Error> {
    match self {
      Self::AssignMembers(command) => command.run().await,
    }
  }
}
