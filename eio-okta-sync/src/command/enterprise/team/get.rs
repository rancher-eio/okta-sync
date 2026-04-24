use crate::github::Enterprise;
use octocrab::Octocrab;

#[derive(Debug, clap::Parser)]
#[remain::sorted]
#[rustfmt::skip]
#[command(about = "get information about an enterprise team")]
pub struct Command {
  #[arg(
    long,
    alias         = "enterprise",
    value_name    = "SLUG",
    default_value = "suse-gmbh",
    help          = "the name enterprise to use"
  )]
  enterprise_slug: String,

  #[arg(
    long,
    alias      = "team",
    value_name = "SLUG",
    help       = "enterprise team slug to delete"
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
}

impl Command {
  #[allow(unused_variables)]
  pub async fn run(self) -> Result<(), crate::Error> {
    let Self {
      enterprise_slug,
      team_slug,
      token,
    } = self;

    let github = Octocrab::builder().personal_token(token).build()?;

    let result = github
      .enterprise(&enterprise_slug)
      .teams()
      .get(&team_slug)
      .send()
      .await?;

    let json = serde_json::to_string_pretty(&result)?;
    println!("{json}");

    Ok(())
  }
}
