use crate::github::graphql::query::enterprise_members::Query;
use crate::graphql::{PaginateQuery, Variables};

#[derive(Debug, clap::Parser)]
#[remain::sorted]
#[rustfmt::skip]
#[command(about = "list users who are members of an enterprise")]
pub struct Command {
  #[arg(
    long,
    alias         = "enterprise",
    value_name    = "SLUG",
    default_value = "suse-gmbh",
    help          = "the enterprise URL slug"
  )]
  enterprise_slug: String,
  #[command(flatten)]
  token: crate::args::github::Token,
}

impl Command {
  #[allow(unused_variables)]
  pub async fn run(self) -> Result<(), crate::Error> {
    let Self { enterprise_slug, token } = self;

    let client = token.try_into()?;

    let variables = Query::variables().enterprise_slug(enterprise_slug).build();
    let response = Query::paginate_query(variables, &client).await?;

    let mut members = Vec::new();

    for member in response {
      members.push(member);
    }

    let json = serde_json::to_string_pretty(&members)?;

    println!("{json}");

    Ok(())
  }
}
