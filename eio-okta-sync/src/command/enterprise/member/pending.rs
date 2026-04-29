mod administrator;
mod affiliated;
mod collaborator;
mod unaffiliated;

#[derive(Debug, clap::Subcommand)]
#[command(about = "list pending GitHub Enterprise invitations")]
#[remain::sorted]
pub enum Command {
  #[clap(aliases = ["administrators", "admin", "admins"])]
  Administrator(administrator::Command),
  #[clap(aliases = ["organization", "org"])]
  Affiliated(affiliated::Command),
  #[clap(aliases = ["repository", "repo", "collaborators", "collab", "collabs"])]
  Collaborator(collaborator::Command),
  #[clap(aliases = ["enterprise"])]
  Unaffiliated(unaffiliated::Command),
}

impl Command {
  pub async fn run(self) -> Result<(), crate::Error> {
    match self {
      Self::Administrator(command) => command.run().await,
      Self::Affiliated(command) => command.run().await,
      Self::Collaborator(command) => command.run().await,
      Self::Unaffiliated(command) => command.run().await,
    }
  }
}
