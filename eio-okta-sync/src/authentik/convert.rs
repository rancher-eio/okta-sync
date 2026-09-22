mod groups;
mod snapshot;
mod users;

#[derive(Debug, Clone, clap::Subcommand)]
#[command(about = "tools for converting Authentik data")]
pub(crate) enum Command {
  Groups(groups::Command),
  Snapshot(snapshot::Command),
  Users(users::Command),
}

impl Command {
  pub(crate) fn run(self) -> Result<(), crate::Error> {
    match self {
      Self::Groups(command) => Ok(command.run()?),
      Self::Snapshot(command) => Ok(command.run()?),
      Self::Users(command) => Ok(command.run()?),
    }
  }
}
