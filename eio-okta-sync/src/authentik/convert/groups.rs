use authentik_client::models::Group as AuthentikGroup;

use crate::{
  authentik::{
    command::io::{Input, Output},
    models::TryIntoOktaGroup,
  },
  okta::Snapshot,
};

#[derive(Debug, Clone, clap::Args)]
#[command(about("converts groups from Authentik format to Okta format"))]
pub(crate) struct Command {
  #[arg(action(clap::ArgAction::Set))]
  #[arg(default_missing_value("true"))]
  #[arg(default_value_t)]
  #[arg(help("wrap the output in a snapshot (with empty users/group_users)?"))]
  #[arg(long)]
  #[arg(num_args(0..=1))]
  #[arg(require_equals(true))]
  #[arg(value_name("BOOL"))]
  as_snapshot: bool,
  #[clap(flatten)]
  input: Input,
  #[clap(flatten)]
  output: Output,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
enum Groups {
  Single(Box<AuthentikGroup>),
  Multiple(Vec<AuthentikGroup>),
}

impl Command {
  pub(crate) fn run(self) -> Result<(), crate::authentik::Error> {
    let Self {
      as_snapshot,
      input,
      output,
    } = self;

    let input_groups = match input.read::<Groups>()? {
      Some(Groups::Multiple(users)) => users,
      Some(Groups::Single(user)) => vec![*user],
      None => Vec::new(),
    };

    let mut output_groups = Vec::with_capacity(input_groups.len());

    for group in input_groups {
      output_groups.push(group.try_into_okta_group()?);
    }

    eprintln!("# converted {} groups", output_groups.len());

    if as_snapshot {
      let snapshot = Snapshot {
        groups: output_groups,
        ..Default::default()
      };
      output.write(&snapshot)?;
    } else {
      output.write(&output_groups)?;
    }

    Ok(())
  }
}
