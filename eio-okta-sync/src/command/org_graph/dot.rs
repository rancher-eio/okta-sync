use camino::Utf8PathBuf;
use petgraph::dot::Dot;

use crate::okta::{Snapshot, graph::Org};

#[derive(educe::Educe, clap::Args)]
#[educe(Debug, Clone, PartialEq, Eq)]
#[remain::sorted]
#[rustfmt::skip]
#[command(about = "generate DOT graph from Okta snapshot")]

pub struct Command {
  #[arg(
    long,
    value_name    = "PATH",
    default_value = "snapshot.yaml",
    help          = "file to read Okta snapshot from"
  )]
  snapshot: Utf8PathBuf,
}

impl Command {
  pub fn run(&self) -> Result<(), crate::Error> {
    let Self { snapshot } = self;

    let snapshot = Snapshot::read_from_file(snapshot)?;

    let org = Org::new(&snapshot.users).populate_descending();

    eprintln!("Node Count: {}", org.hierarchy.node_count());

    let dot = Dot::new(&org.hierarchy);

    println!("{dot}");

    Ok(())
  }
}
