mod assign_members;
mod create;
mod delete;
mod get;
mod list;
mod member;
mod organization;

#[derive(Debug, clap::Subcommand)]
#[command(about = "tools for managing GitHub Enterprise Teams")]
pub enum Command {
  AssignMembers(assign_members::Command),
  #[command(subcommand, alias = "organizations")]
  Organization(organization::Command),
  Create(create::Command),
  Delete(delete::Command),
  Get(get::Command),
  List(list::Command),
  #[command(subcommand, alias = "members")]
  Member(member::Command),
}

impl Command {
  pub async fn run(self) -> Result<(), crate::Error> {
    match self {
      Self::AssignMembers(command) => command.run().await,
      Self::Organization(command) => command.run().await,
      Self::Create(command) => command.run().await,
      Self::Delete(command) => command.run().await,
      Self::Get(command) => command.run().await,
      Self::List(command) => command.run().await,
      Self::Member(command) => command.run().await,
    }
  }
}
