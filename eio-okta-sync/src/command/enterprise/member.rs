mod invite;

#[derive(Debug, clap::Subcommand)]
#[command(about = "tools for managing GitHub Enterprise Teams")]
pub enum Command {
  Invite(invite::Command),
}

impl Command {
  pub async fn run(self) -> Result<(), crate::Error> {
    match self {
      Self::Invite(command) => command.run().await,
    }
  }
}
