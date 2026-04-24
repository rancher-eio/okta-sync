use crate::github::Enterprise;
use octocrab::Octocrab;

#[derive(Debug, clap::Parser)]
#[remain::sorted]
#[rustfmt::skip]
#[command(about = "remove multiple users from an enterprise team")]
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

  #[arg(
    long,
    value_delimiter = ',',
    value_name = "USERNAME,USERNAME,...",
    help       = "comma-separated GitHub usernames to remove from the team"
  )]
  usernames: Vec<String>,
}

impl Command {
  pub async fn run(self) -> Result<(), crate::Error> {
    let Self {
      enterprise_slug,
      team_slug,
      token,
      usernames,
    } = self;

    let github = Octocrab::builder().personal_token(token).build()?;
    let count = usernames.len();
    github
      .enterprise(&enterprise_slug)
      .team(&team_slug)
      .memberships()
      .bulk_remove()
      .usernames(usernames)
      .build()
      .send()
      .await?;

    println!("Successfully removed {} members from team '{}'", count, team_slug);

    Ok(())
  }
}
