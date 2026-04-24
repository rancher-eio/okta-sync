use crate::github::Enterprise;
use octocrab::Octocrab;

#[derive(Debug, clap::Parser)]
#[remain::sorted]
#[rustfmt::skip]
#[command(about = "add a user to an enterprise team")]
pub struct Command {
  #[arg(
    long,
    value_name    = "SLUG",
    default_value = "suse-gmbh",
    help          = "name of enterprise to use"
  )]
  enterprise_slug: String,

  #[arg(
    long,
    alias      = "team",
    value_name = "SLUG",
    help       = "enterprise team slug",
  )]
  team_slug: String,

  #[arg(
    env        = "GITHUB_TOKEN",
    long       = "token",
    value_name = "TOKEN",
    help       = "GitHub Access Token",
    hide_env_values = true,
  )]
  token: String,

  #[arg(
    long,
    alias      = "user",
    value_name = "USERNAME",
    help       = "GitHub username to add to the team"
  )]
  username: String,
}

impl Command {
  pub async fn run(self) -> Result<(), crate::Error> {
    let Self {
      enterprise_slug,
      team_slug,
      token,
      username,
    } = self;

    let github = Octocrab::builder().personal_token(token).build()?;

    let result = github
      .enterprise(&enterprise_slug)
      .team(&team_slug)
      .memberships()
      .add(&username)
      .send()
      .await?;

    let json = serde_json::to_string_pretty(&result)?;
    println!("{json}");

    Ok(())
  }
}
