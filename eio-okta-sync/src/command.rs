mod check_version;
mod current;
mod generate;
mod make_archive;
mod mappings;
mod members;
mod snapshot;

use clap::Parser;

#[derive(Debug, Parser)]
#[remain::sorted]
#[command(about = "a tool for populating GitHub Orgs from Okta Users/Groups")]
pub enum Command {
  CheckVersion(check_version::Command),
  Current(current::Command),
  Generate(generate::Command),
  MakeArchive(make_archive::Command),
  Mappings(mappings::Command),
  Members(members::Command),
  Snapshot(snapshot::Command),
}

impl Command {
  pub async fn run(self) -> Result<(), crate::Error> {
    match self {
      Self::CheckVersion(command) => command.run(),
      Self::Current(command) => command.run(),
      Self::Generate(command) => command.run(),
      Self::MakeArchive(command) => command.run(),
      Self::Mappings(command) => command.run(),
      Self::Members(command) => command.run().await,
      Self::Snapshot(command) => command.run(),
    }
  }
}
