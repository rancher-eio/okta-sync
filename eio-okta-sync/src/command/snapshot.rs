use crate::okta::Snapshot;
use camino::Utf8PathBuf;
use clap::Parser;
use eio_okta_api::v1::{
  groups::{ListGroupUsers, ListGroups},
  users::ListUsers,
};
use eio_okta_client::Client;
use eio_okta_data::v2024_07_0::management::components::parameters::{PathGroupId, QueryLimit};

#[derive(Debug, Parser)]
#[remain::sorted]
#[rustfmt::skip]
#[command(
  about      = "collects a snapshot of the current state of Okta",
  long_about = "collects a snapshot of the current state of Okta.

This will collect all users and groups visible to the authorization token used."
)]
#[group(skip)]
pub struct Command {
  #[command(flatten)]
  client: eio_okta_client::Options,
  #[arg(
    long,
    value_name    = "PATH",
    default_value = "snapshot.yaml",
    help          = "file to write output to",
  )]
  output: Utf8PathBuf,
}

impl Command {
  #[remain::check]
  pub fn run(self) -> Result<(), crate::Error> {
    let Command { client, output } = self;

    let client = Client::from(client);

    let mut snapshot = Snapshot::default();

    let limit = 200;

    eprintln!("collecting users...");
    snapshot.users = client.paginate(ListUsers {
      limit: QueryLimit { limit },
      ..Default::default()
    })?;

    eprintln!("collecting groups...");
    snapshot.groups = client.paginate(ListGroups {
      limit: QueryLimit { limit },
      ..Default::default()
    })?;

    for group in snapshot.groups.iter() {
      let group_id = group.id.clone();
      eprintln!("collecting members for group {group_id}...");
      let members = client.paginate(ListGroupUsers {
        path: PathGroupId { group_id },
        query: QueryLimit { limit },
      })?;
      snapshot.group_users.insert(group.id.clone(), members);
    }

    let yaml = serde_yml::to_string(&snapshot)?;

    fs_err::write(&output, yaml)?;

    eprintln!("\n[OK] snapshot saved to {output}");

    Ok(())
  }
}
