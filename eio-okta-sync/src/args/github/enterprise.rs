use crate::github::graphql::query::enterprise::Query;
use crate::graphql::{ExecuteQuery, Variables};

#[derive(Debug, clap::Args)]
#[group(id = "enterprise", required = true, multiple = false)]
#[remain::sorted]
#[rustfmt::skip]
pub(crate) struct EnterpriseIdOrSlug {
  #[arg(
    id         = "enterprise.id",
    long       = "enterprise-id",
    value_name = "ID",
    help       = "the ID of the enterprise",
  )]
  pub(crate) id: Option<String>,
  #[arg(
    id         = "enterprise.slug",
    long       = "enterprise-slug",
    value_name = "SLUG",
    help       = "the enterprise URL slug, to look up the ID (at the cost of an additional API call)",
  )]
  pub(crate) slug: Option<String>,
}

impl EnterpriseIdOrSlug {
  pub(crate) async fn id(self, client: &octocrab::Octocrab) -> Result<String, crate::Error> {
    if let Some(value) = self.id {
      Ok(value)
    } else if let Some(value) = self.slug {
      let variables = Query::variables().enterprise_slug(value).build();
      let response = Query::execute_query(variables, client).await?;

      if let Some(errors) = response.errors {
        Err(crate::Error::GraphQL(
          errors.into_iter().map(|error| error.message).collect(),
        ))
      } else {
        let value = response
          .data
          .enterprise
          .ok_or_else(|| crate::Error::DispleasingResponseData {
            reason: "enterprise details are missing",
          })?
          .id;

        Ok(value)
      }
    } else {
      Err(crate::Error::ArgumentValidationFailedToPrevent {
        betrayal: "neither --enterprise-id nor --enterprise-slug was provided",
      })
    }
  }
}
