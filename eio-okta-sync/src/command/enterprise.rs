mod cost_center;
mod list;
mod member;
mod organization;
mod team;

#[derive(Debug, clap::Subcommand)]
#[command(about = "tools for working with GitHub Enterprise")]
#[remain::sorted]
pub enum Command {
  #[command(subcommand)]
  CostCenter(cost_center::Command),
  List(list::Command),
  #[command(subcommand, alias = "members")]
  Member(member::Command),
  #[command(subcommand, alias = "organizations")]
  Organization(organization::Command),
  #[command(subcommand, alias = "teams")]
  Team(team::Command),
}

impl Command {
  pub async fn run(self) -> Result<(), crate::Error> {
    match self {
      Self::CostCenter(command) => command.run().await,
      Self::List(command) => command.run().await,
      Self::Member(command) => command.run().await,
      Self::Organization(command) => command.run().await,
      Self::Team(command) => command.run().await,
    }
  }
}
