use authentik_client::apis::Error;
use authentik_client::apis::configuration::Configuration;
use authentik_client::apis::core_api::{CoreGroupsDestroyError, core_groups_destroy};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, clap::Parser, serde::Serialize, serde::Deserialize, bon::Builder)]
#[command(about = "Destroy a group")]
#[command(after_help = "API Reference @ https://api.goauthentik.io/reference/core-groups-destroy/")]
#[remain::sorted]
pub struct CoreGroupsDestroy {
  #[arg(help_heading = "Query Parameters")]
  #[arg(long)]
  #[arg(value_name = "UUID")]
  group_uuid: Uuid,
}

impl crate::authentik::traits::GetWithConfiguration for CoreGroupsDestroy {
  type Error = Error<CoreGroupsDestroyError>;
  type Value = ();

  async fn get_with_configuration(self, configuration: &Configuration) -> Result<Self::Value, Self::Error> {
    let Self { group_uuid } = self;

    core_groups_destroy(configuration, &group_uuid.to_string()).await
  }
}

crate::authentik::macros::RunAsync!(CoreGroupsDestroy as Wrapper<GetWithConfiguration>);

crate::authentik::macros::From!(
  Error<CoreGroupsDestroyError>
  => super::CoreGroupsApiError
  => super::super::CoreApiError
  => super::super::super::ApiError
  => super::super::super::super::Error
);
