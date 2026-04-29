mod cancel_invitation;
mod invite;
mod list;
mod pending;
mod remove;

#[derive(Debug, clap::Subcommand)]
#[command(about = "tools for managing GitHub Enterprise Members")]
#[remain::sorted]
pub enum Command {
  CancelInvitation(cancel_invitation::Command),
  Invite(invite::Command),
  List(list::Command),
  #[command(subcommand)]
  Pending(pending::Command),
  Remove(remove::Command),
}

impl Command {
  pub async fn run(self) -> Result<(), crate::Error> {
    match self {
      Self::CancelInvitation(command) => command.run().await,
      Self::Invite(command) => command.run().await,
      Self::List(command) => command.run().await,
      Self::Pending(command) => command.run().await,
      Self::Remove(command) => command.run().await,
    }
  }
}
