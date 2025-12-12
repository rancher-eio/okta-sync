mod cost_center;
mod team;

#[derive(Debug, clap::Subcommand)]
#[command(about = "tools for working with GitHub Enterprise")]
pub enum Command {
  #[command(subcommand)]
  CostCenter(cost_center::Command),
  #[command(subcommand)]
  Team(team::Command),
}

impl Command {
  pub async fn run(self) -> Result<(), crate::Error> {
    match self {
      Self::CostCenter(command) => command.run().await,
      Self::Team(command) => command.run().await,
    }
  }
}
