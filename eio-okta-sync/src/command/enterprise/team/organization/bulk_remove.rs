use crate::github::Enterprise;
use octocrab::Octocrab;

#[derive(Debug, clap::Parser)]
#[remain::sorted]
#[rustfmt::skip]
#[command(about = "remove mulitple organizations from an enterprise team")]
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
    value_delimiter = ',',
    value_name = "SLUG,SLUG,...",
    help       = "comma-separated organization slugs to remove from the team"
  )]
  org_slugs: Vec<String>,
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
      org_slugs,
      team_slug,
      token,
    } = self;

    let github = Octocrab::builder().personal_token(token).build()?;

    github
      .enterprise(&enterprise_slug)
      .team(&team_slug)
      .organizations()
      .bulk_remove()
      .orgs(org_slugs.clone())
      .build()
      .send()
      .await?;
    println!(
      "Successfully removed organizations '{}' from team '{}'",
      org_slugs.len(),
      team_slug
    );

    Ok(())
  }
}
