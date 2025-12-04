mod dot;
mod path;

use clap::Parser;

#[derive(Debug, Parser)]
#[remain::sorted]
#[command(about = "commands for working with org as graph data")]
pub enum Command {
  Dot(dot::Command),
  Path(path::Command),
}

impl Command {
  pub fn run(self) -> Result<(), crate::Error> {
    match self {
      Self::Dot(command) => command.run(),
      Self::Path(command) => command.run(),
    }
  }
}
