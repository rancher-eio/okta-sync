use std::collections::BTreeSet;

use octocrab::Octocrab;

use crate::github::graphql::query::viewer_enterprises::Query;
use crate::graphql::{PaginateQuery, Variables};

#[derive(Debug, clap::Parser)]
#[remain::sorted]
#[rustfmt::skip]
#[command(about = "list the enterprises that the user belongs to")]
pub struct Command {
  #[arg(
    alias = "filter",
    long,
    value_name = "FILTER",
    value_delimiter = ',',
    help = "filters to apply to output"
  )]
  filters: Vec<Filter>,
  #[arg(
    long,
    help = "bypass all processing and dump raw response JSON",
  )]
  raw: bool,
  #[command(flatten)]
  token: crate::args::github::Token,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, clap::ValueEnum)]
#[clap(rename_all = "camelCase")]
#[remain::sorted]
pub enum Filter {
  IsAdmin,
}

impl Command {
  #[allow(unused_variables)]
  pub async fn run(self) -> Result<(), crate::Error> {
    let Self { filters, raw, token } = self;

    let filters = BTreeSet::from_iter(filters);

    let github = Octocrab::builder().personal_token(token).build()?;

    let variables = Query::variables().build();
    let response = Query::paginate_query(variables, &github).await?;

    let output = if raw {
      serde_json::to_string_pretty(&response)?
    } else {
      let mut enterprises = BTreeSet::new();

      for enterprise in response {
        if enterprise.viewer_is_admin || !filters.contains(&Filter::IsAdmin) {
          enterprises.insert(enterprise);
        }
      }

      serde_json::to_string_pretty(&enterprises)?
    };

    println!("{output}");

    Ok(())
  }
}
