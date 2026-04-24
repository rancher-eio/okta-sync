use octocrab::Octocrab;

use crate::github::Enterprise;

#[derive(Debug, clap::Parser)]
#[remain::sorted]
#[rustfmt::skip]
#[command(about = "delete an enterprise team")]
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
  pub async fn run(self) -> Result<(), crate::Error> {
    let Self {
      enterprise_slug,
      team_slug,
      token,
    } = self;

    let github = Octocrab::builder().personal_token(token).build()?;

    github
      .enterprise(&enterprise_slug)
      .teams()
      .delete(&team_slug)
      .send()
      .await?;

    println!("Successfully deleted team '{}'", team_slug);

    Ok(())
  }
}
