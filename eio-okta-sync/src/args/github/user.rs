use crate::github::graphql::query::user::Query;
use crate::graphql::{ExecuteQuery, Variables};

#[derive(Debug, clap::Args)]
#[group(id = "user", required = true, multiple = false)]
#[remain::sorted]
#[rustfmt::skip]
pub(crate) struct UserIdOrLogin {
  #[arg(
    id         = "user.id",
    long       = "user-id",
    value_name = "ID",
    help       = "the ID of the user",
  )]
  pub(crate) id: Option<String>,
  #[arg(
    id         = "user.login",
    long       = "user-login",
    value_name = "LOGIN",
    help       = "the login of the user, to look up the ID (at the cost of an additional API call)",
  )]
  pub(crate) login: Option<String>,
}

impl UserIdOrLogin {
  pub(crate) async fn id(self, client: &octocrab::Octocrab) -> Result<String, crate::Error> {
    if let Some(value) = self.id {
      Ok(value)
    } else if let Some(value) = self.login {
      let variables = Query::variables().user_login(value).build();
      let response = Query::execute_query(variables, client).await?;
      if let Some(errors) = response.errors {
        Err(crate::Error::GraphQL(
          errors.into_iter().map(|error| error.message).collect(),
        ))
      } else {
        let value = response
          .data
          .user
          .ok_or_else(|| crate::Error::DispleasingResponseData {
            reason: "user details are missing",
          })?
          .id;

        Ok(value)
      }
    } else {
      Err(crate::Error::ArgumentValidationFailedToPrevent {
        betrayal: "neither --user-id nor --user-login was provided",
      })
    }
  }
}
