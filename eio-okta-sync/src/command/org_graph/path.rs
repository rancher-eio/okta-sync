use camino::Utf8PathBuf;
use petgraph::{algo, visit::NodeIndexable};
use std::{collections::BTreeSet, io::Read, time::Instant};

use crate::okta::{DisplayName, Snapshot, graph::Org};

#[derive(educe::Educe, clap::Args)]
#[educe(Debug, Clone, PartialEq, Eq)]
#[remain::sorted]
#[rustfmt::skip]
#[command(about = "find path to user(s) in Okta snapshot")]

pub struct Command {
  #[arg(
    long,
    help = "dump all users",
    conflicts_with_all(["stats", "stdin", "user"]),
  )]
  all: bool,
  #[arg(
    long,
    value_name = "DIRECTION",
    default_value = "down",
    help = "discovery method to use when constructing org graph",
  )]
  discovery: Discovery,
  #[arg(
    long,
    help = "output names instead of Okta IDs when reading from STDIN"
  )]
  names: bool,
  #[arg(
    long,
    value_name    = "ACTION",
    default_value = "exit",
    help          = "action to take when failing to find a path"
  )]
  on_failure: OnFailure,
  #[arg(
    long,
    value_name    = "ACTION",
    default_value = "exit",
    help          = "action to take when a user is missing"
  )]
  on_missing: OnMissing,
  #[arg(
    long,
    value_name    = "PATH",
    default_value = "snapshot.yaml",
    help          = "file to read Okta snapshot from"
  )]
  snapshot: Utf8PathBuf,
  #[arg(
    long,
    value_name = "OKTA-ID",
    help = "start at this employee (defaults to top of org)",
  )]
  start: Option<String>,
  #[arg(
    long,
    help = "dump stats and exit",
    conflicts_with_all(["all", "stdin", "user"])
  )]
  stats: bool,
  #[arg(
    long,
    help = "read users from STDIN"
  )]
  stdin: bool,
  #[arg(
    long,
    value_name = "OKTA-ID",
    help = "find path to this employee",
    required_unless_present_any(["all", "stats", "stdin"])
  )]
  user: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, clap::ValueEnum)]
#[clap(rename_all = "kebab-case")]
#[remain::sorted]
enum Discovery {
  #[default]
  #[value(
    help = "starts at the top of the org (users who are their own manager), and walks their direct reports. Useful with traditional org hierarchies."
  )]
  Down,
  #[value(
    help = "combines 'down' with 'up', at the cost of being as slow as doing both (because that's what it does). Useful with mostly-traditional org hierarchies with some disconnected reporting structures."
  )]
  DownUp,
  #[value(
    help = "naively iterates over all users in the dataset. Tends to include bots and other entities outside the org hierarchy."
  )]
  Naive,
  #[value(
    help = "starts at the bottom of the org (users with no direct reports), and walks up the management chain. Useful with disconnected org hierarchies."
  )]
  Up,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, clap::ValueEnum)]
#[clap(rename_all = "kebab-case")]
#[remain::sorted]
enum OnFailure {
  #[default]
  Exit,
  Skip,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, clap::ValueEnum)]
#[clap(rename_all = "kebab-case")]
#[remain::sorted]
enum OnMissing {
  #[default]
  Exit,
  Skip,
}

impl Command {
  pub fn run(&self) -> Result<(), crate::Error> {
    let Self {
      all,
      discovery,
      names,
      on_failure,
      on_missing,
      snapshot,
      start,
      stats,
      stdin,
      user,
    } = self;

    let init_time = Instant::now();

    let snapshot = Snapshot::read_from_file(snapshot)?;

    let snapshot_time = Instant::now();

    let org = {
      let org = Org::new(&snapshot.users);
      match discovery {
        Discovery::Down => org.populate_descending(),
        Discovery::DownUp => org.populate_descending().populate_ascending(),
        Discovery::Naive => org.populate_naive(),
        Discovery::Up => org.populate_ascending(),
      }
    };

    let discovery_time = Instant::now();

    if *stats {
      println!("Snapshot Load Time: {:?}", snapshot_time - init_time);
      println!("Snapshot Users: {}", snapshot.users.len());
      println!("Discovery Time: {:?}", discovery_time - snapshot_time);
      println!("Discovered Nodes: {}", org.hierarchy.node_count());
      println!("Discovered Edges: {}", org.hierarchy.edge_count());

      return Ok(());
    }

    let start = start
      .as_ref()
      .cloned()
      .unwrap_or_else(|| org.hierarchy.from_index(0).to_owned());

    if org.user(&start).is_none() {
      eprintln!("starting user not found in discovered org hierarchy: {start}");
      std::process::exit(1);
    }

    let users: BTreeSet<String> = if *all {
      let mut users = vec![start.as_str()];
      users.extend(org.below(&start));
      users.into_iter().map(ToOwned::to_owned).collect()
    } else if *stdin {
      let mut buf = String::new();
      std::io::stdin().read_to_string(&mut buf)?;
      buf.split_whitespace().map(ToOwned::to_owned).collect()
    } else {
      let user = user
        .as_ref()
        .cloned()
        .expect("Missing user argument. CLI argument validation should have prevented this. (╯°□°)╯︵ ┻━┻");
      vec![user].into_iter().collect()
    };

    let mut missing = BTreeSet::new();

    for id in &users {
      if org.user(id).is_none() {
        let context = match snapshot.users.iter().find(|user| user.id.eq(id)) {
          Some(_user) => "present in snapshot",
          None => "missing in snapshot",
        };

        eprintln!("user not found in discovered org hierarchy ({context}): {id}");
        missing.insert(id.clone());
      }
    }

    if !missing.is_empty() && *on_missing == OnMissing::Exit {
      std::process::exit(2);
    };

    let edge_cost = |_| 1;
    let estimate_cost = |_| 0;

    for goal in users.difference(&missing) {
      let is_goal = |node| node == goal;
      match algo::astar(&org.hierarchy, &start, is_goal, edge_cost, estimate_cost) {
        None => {
          eprintln!("failed to find path from '{start}' to '{goal}'");
          match on_failure {
            OnFailure::Skip => continue,
            OnFailure::Exit => std::process::exit(3),
          }
        }
        Some((_cost, path)) => {
          if *all || *stdin {
            if *names {
              let mut names = Vec::with_capacity(path.len());

              for id in &path {
                let user = org.user(id).unwrap();
                names.push(user.display_name());
              }

              println!("{}", names.join("\t"));
            } else {
              println!("{}", path.join("\t"));
            }
          } else {
            for id in &path {
              let user = org.user(id).unwrap();
              println!("{id} {}", user.display_name());
            }
          }
        }
      }
    }

    Ok(())
  }
}
