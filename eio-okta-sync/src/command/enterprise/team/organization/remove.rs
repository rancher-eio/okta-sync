use crate::github::Enterprise;
use octocrab::Octocrab;

#[derive(Debug, clap::Parser)]
#[remain::sorted]
#[rustfmt::skip]
#[command(about = "remove an organization from an enterprise team")]
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
    alias       = "org",
    value_name = "SLUG",
    help       = "organization slug to assign to the team",
  )]
  org_slug: String,
  #[arg(
    long,
    alias       = "team",
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
}

impl Command {
  #[allow(unused_variables)]
  pub async fn run(self) -> Result<(), crate::Error> {
    let Self {
      enterprise_slug,
      org_slug,
      team_slug,
      token,
    } = self;

    let github = Octocrab::builder().personal_token(token).build()?;

    let result = github
      .enterprise(&enterprise_slug)
      .team(&team_slug)
      .organizations()
      .remove(&org_slug)
      .send()
      .await?;
    println!(
      "Successfully removed organization '{}' from team '{}'",
      org_slug, team_slug
    );

    Ok(())
  }
}
