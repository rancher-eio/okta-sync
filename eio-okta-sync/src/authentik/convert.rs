mod users;

#[derive(Debug, Clone, clap::Subcommand)]
#[command(about = "tools for converting Authentik data")]
pub(crate) enum Command {
  Users(users::Command),
}

impl Command {
  pub(crate) fn run(self) -> Result<(), crate::Error> {
    match self {
      Self::Users(command) => Ok(command.run()?),
    }
  }
}
