use crate::github::Enterprise;
use bounded_integer::BoundedU8;
use octocrab::Octocrab;

#[derive(Debug, clap::Parser)]
#[remain::sorted]
#[rustfmt::skip]
#[command(about = "list enterprise teams")]
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
    let Self { enterprise_slug, token } = self;

    let github = Octocrab::builder().personal_token(token).build()?;
    let per_page = BoundedU8::MAX;
    let teams = github
      .all_pages(
        github
          .enterprise(&enterprise_slug)
          .teams()
          .list()
          .per_page(per_page)
          .build()
          .send()
          .await?,
      )
      .await?;
    let json = serde_json::to_string_pretty(&teams)?;
    println!("{json}");
    Ok(())
  }
}
