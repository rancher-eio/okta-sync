use octocrab::Octocrab;

use crate::github::graphql::query::enterprise_pending_admin_invitations::Query;
use crate::graphql::{PaginateQuery, Variables};

#[derive(Debug, clap::Parser)]
#[remain::sorted]
#[rustfmt::skip]
#[command(about = "list pending administrator invitations for the enterprise")]
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
      let mut invitations = Vec::new();

      for invitation in response {
        invitations.push(invitation);
      }

      serde_json::to_string_pretty(&invitations)?
    };

    println!("{output}");

    Ok(())
  }
}
