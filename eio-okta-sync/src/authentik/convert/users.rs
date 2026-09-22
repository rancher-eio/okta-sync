use authentik_client::models::User as AuthentikUser;

use crate::{
  authentik::{
    command::io::{Input, Output},
    models::TryIntoOktaUser,
  },
  okta::Snapshot,
};

#[derive(Debug, Clone, clap::Args)]
#[command(about("converts users from Authentik format to Okta format"))]
pub(crate) struct Command {
  #[arg(action(clap::ArgAction::Set))]
  #[arg(default_missing_value("true"))]
  #[arg(default_value_t)]
  #[arg(help("wrap the output in a snapshot (with empty groups/group_users)?"))]
  #[arg(long)]
  #[arg(num_args(0..=1))]
  #[arg(require_equals(true))]
  #[arg(value_name("BOOL"))]
  as_snapshot: bool,
  #[arg(help("override profile.githubOrgs for converted user(s)"))]
  #[arg(long)]
  #[arg(value_delimiter(','))]
  github_orgs: Vec<String>,
  #[clap(flatten)]
  input: Input,
  #[clap(flatten)]
  output: Output,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
enum Users {
  Single(Box<AuthentikUser>),
  Multiple(Vec<AuthentikUser>),
}

impl Command {
  pub(crate) fn run(self) -> Result<(), crate::authentik::Error> {
    let Self {
      as_snapshot,
      github_orgs,
      input,
      output,
    } = self;

    let extensions = [(String::from("githubOrgs"), serde_json::to_value(github_orgs)?)];

    let input_users = match input.read::<Users>()? {
      Some(Users::Multiple(users)) => users,
      Some(Users::Single(user)) => vec![*user],
      None => Vec::new(),
    };

    let mut output_users = Vec::with_capacity(input_users.len());

    for user in input_users {
      let mut user = user.try_into_okta_user()?;
      user.profile._extensions.extend(extensions.iter().cloned());
      output_users.push(user);
    }

    eprintln!("# converted {} users", output_users.len());

    if as_snapshot {
      let snapshot = Snapshot {
        users: output_users,
        ..Default::default()
      };
      output.write(&snapshot)?;
    } else {
      output.write(&output_users)?;
    }

    Ok(())
  }
}
