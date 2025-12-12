mod assign_team;

#[derive(Debug, clap::Subcommand)]
#[command(about = "tools for managing GitHub Enterprise Cost Centers")]
pub enum Command {
  AssignTeam(assign_team::Command),
}

impl Command {
  pub async fn run(self) -> Result<(), crate::Error> {
    match self {
      Self::AssignTeam(command) => command.run().await,
    }
  }
}
