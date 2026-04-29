use octocrab::Octocrab;

use crate::github::graphql::query::enterprise_organizations::Query;
use crate::graphql::{PaginateQuery, Variables};

#[derive(Debug, clap::Parser)]
#[remain::sorted]
#[rustfmt::skip]
#[command(about = "list organizations that belong to an enterprise")]
pub struct Command {
  #[arg(
    long,
    alias         = "enterprise",
    value_name    = "SLUG",
    default_value = "suse-gmbh",
    help          = "the enterprise URL slug"
  )]
  enterprise_slug: String,
  #[arg(
    long,
    help = "bypass all processing and dump raw response JSON",
  )]
  raw: bool,
  #[command(flatten)]
  token: crate::args::github::Token,
}

impl Command {
  #[allow(unused_variables)]
  pub async fn run(self) -> Result<(), crate::Error> {
    let Self {
      enterprise_slug,
      raw,
      token,
    } = self;

    let github = Octocrab::builder().personal_token(token).build()?;

    let variables = Query::variables().enterprise_slug(enterprise_slug).build();
    let response = Query::paginate_query(variables, &github).await?;

    let output = if raw {
      serde_json::to_string_pretty(&response)?
    } else {
      let mut organizations = Vec::new();

      for organization in response {
        organizations.push(organization);
      }

      serde_json::to_string_pretty(&organizations)?
    };

    println!("{output}");

    Ok(())
  }
}
