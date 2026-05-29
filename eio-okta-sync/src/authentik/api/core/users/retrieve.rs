use authentik_client::apis::Error;
use authentik_client::apis::configuration::Configuration;
use authentik_client::apis::core_api::{CoreUsersRetrieveError, core_users_retrieve};
use authentik_client::models::User;

#[derive(Debug, Clone, PartialEq, Eq, clap::Parser, serde::Serialize, serde::Deserialize, bon::Builder)]
#[command(about = "Get information about a specific user")]
#[command(after_help = "API Reference @ https://api.goauthentik.io/reference/core-users-retrieve/")]
#[remain::sorted]
pub struct CoreUsersRetrieve {
  #[arg(help_heading = "Query Parameters")]
  #[arg(long)]
  #[arg(value_name = "INTEGER")]
  id: i32,
}

impl crate::authentik::traits::GetWithConfiguration for CoreUsersRetrieve {
  type Error = Error<CoreUsersRetrieveError>;
  type Value = User;

  async fn get_with_configuration(self, configuration: &Configuration) -> Result<Self::Value, Self::Error> {
    let Self { id } = self;

    core_users_retrieve(configuration, id).await
  }
}

crate::authentik::macros::RunAsync!(CoreUsersRetrieve as Wrapper<GetWithConfiguration>);

crate::authentik::macros::From!(
  Error<CoreUsersRetrieveError>
  => super::CoreUsersApiError
  => super::super::CoreApiError
  => super::super::super::ApiError
  => super::super::super::super::Error
);
