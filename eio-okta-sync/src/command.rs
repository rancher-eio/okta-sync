mod current;
mod generate;
mod members;
mod snapshot;

use clap::Parser;

#[derive(Debug, Parser)]
#[remain::sorted]
#[command(about = "a tool for populating GitHub Orgs from Okta Users/Groups")]
pub enum Command {
  Current(current::Command),
  Generate(generate::Command),
  Members(members::Command),
  Snapshot(snapshot::Command),
}

impl Command {
  pub async fn run(self) -> Result<(), crate::Error> {
    match self {
      Self::Current(command) => command.run(),
      Self::Generate(command) => command.run(),
      Self::Members(command) => command.run().await,
      Self::Snapshot(command) => command.run(),
    }
  }
}
