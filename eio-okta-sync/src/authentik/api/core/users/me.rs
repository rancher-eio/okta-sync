use authentik_client::apis::Error;
use authentik_client::apis::configuration::Configuration;
use authentik_client::apis::core_api::{CoreUsersMeRetrieveError, core_users_me_retrieve};
use authentik_client::models::SessionUser;

#[derive(Debug, Clone, PartialEq, Eq, clap::Parser, serde::Serialize, serde::Deserialize, bon::Builder)]
#[command(about = "Get information about the current user")]
#[command(after_help = "API Reference @ https://api.goauthentik.io/reference/core-users-me-retrieve/")]
#[remain::sorted]
pub struct CoreUsersMeRetrieve {}

impl crate::authentik::traits::GetWithConfiguration for CoreUsersMeRetrieve {
  type Error = Error<CoreUsersMeRetrieveError>;
  type Value = SessionUser;

  async fn get_with_configuration(self, configuration: &Configuration) -> Result<Self::Value, Self::Error> {
    core_users_me_retrieve(configuration).await
  }
}

crate::authentik::macros::Default!(CoreUsersMeRetrieve as Builder);
crate::authentik::macros::RunAsync!(CoreUsersMeRetrieve as Wrapper<GetWithConfiguration>);

crate::authentik::macros::From!(
  Error<CoreUsersMeRetrieveError>
  => super::CoreUsersApiError
  => super::super::CoreApiError
  => super::super::super::ApiError
  => super::super::super::super::Error
);
