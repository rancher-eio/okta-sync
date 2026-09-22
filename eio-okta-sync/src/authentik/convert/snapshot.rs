use std::collections::BTreeMap;

use authentik_client::models::{Group as AuthentikGroup, User as AuthentikUser};
use camino::Utf8PathBuf;
use indexmap::IndexMap;
use itertools::Itertools;

use crate::{
  authentik::{
    command::io::{Input, InputFormat, InputSource, Output},
    models::{TryIntoOktaGroup, TryIntoOktaUser},
  },
  okta::Snapshot,
};

#[derive(Debug, Clone, clap::Args)]
#[command(about("converts users and groups from Authentik to an Okta-like snapshot"))]
pub(crate) struct Command {
  #[arg(default_value = <&'static str>::from(InputFormat::default()))]
  input_format: InputFormat,
  #[arg(default_value = "groups.json")]
  #[arg(long)]
  #[arg(value_name = "PATH")]
  input_groups: Utf8PathBuf,
  #[arg(default_value = "users.json")]
  #[arg(long)]
  #[arg(value_name = "PATH")]
  input_users: Utf8PathBuf,
  #[clap(flatten)]
  output: Output,
}

impl Command {
  pub(crate) fn run(self) -> Result<(), crate::authentik::Error> {
    let Self {
      input_format,
      input_groups,
      input_users,
      output,
    } = self;

    let input_groups = Input::builder()
      .source(InputSource::Path(input_groups))
      .format(input_format)
      .build()
      .read::<Vec<AuthentikGroup>>()?
      .unwrap_or_default();

    let input_users = Input::builder()
      .source(InputSource::Path(input_users))
      .format(input_format)
      .build()
      .read::<Vec<AuthentikUser>>()?
      .unwrap_or_default();

    let users_by_pk = input_users
      .into_iter()
      .map(|user| {
        let pk = user.pk;
        user.try_into_okta_user().map(|user| (pk, user))
      })
      .collect::<Result<IndexMap<_, _>, _>>()?;

    let mut groups = Vec::with_capacity(input_groups.len());
    let mut group_users = BTreeMap::new();

    for group in input_groups {
      let users = group
        .users
        .iter()
        .flatten()
        .filter_map(|pk| users_by_pk.get(pk))
        .cloned()
        .sorted_by(|a, b| a.id.cmp(&b.id))
        .collect_vec();
      let group = group.try_into_okta_group()?;
      group_users.insert(group.id.clone(), users);
      groups.push(group);
    }

    groups.sort_by(|a, b| a.id.cmp(&b.id));

    let users = users_by_pk.into_values().collect_vec();

    eprintln!("# converted {} users and {} groups", users.len(), groups.len());

    let snapshot = Snapshot {
      groups,
      group_users,
      users,
    };

    output.write(&snapshot)?;

    Ok(())
  }
}
