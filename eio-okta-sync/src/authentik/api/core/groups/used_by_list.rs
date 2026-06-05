use authentik_client::apis::Error;
use authentik_client::apis::configuration::Configuration;
use authentik_client::apis::core_api::{CoreGroupsUsedByListError, core_groups_used_by_list};
use authentik_client::models::UsedBy;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, clap::Parser, serde::Serialize, serde::Deserialize, bon::Builder)]
#[command(about = "Get a list of all objects that use this object")]
#[command(after_help = "API Reference @ https://api.goauthentik.io/reference/core-groups-used-by-list/")]
#[remain::sorted]
pub struct CoreGroupsUsedByList {
  #[arg(help_heading = "Query Parameters")]
  #[arg(long)]
  #[arg(value_name = "UUID")]
  group_uuid: Uuid,
}

impl crate::authentik::traits::GetWithConfiguration for CoreGroupsUsedByList {
  type Error = Error<CoreGroupsUsedByListError>;
  type Value = Vec<UsedBy>;

  async fn get_with_configuration(self, configuration: &Configuration) -> Result<Self::Value, Self::Error> {
    let Self { group_uuid } = self;

    core_groups_used_by_list(configuration, &group_uuid.to_string()).await
  }
}

crate::authentik::macros::RunAsync!(CoreGroupsUsedByList as Wrapper<GetWithConfiguration>);

crate::authentik::macros::From!(
  Error<CoreGroupsUsedByListError>
  => super::CoreGroupsApiError
  => super::super::CoreApiError
  => super::super::super::ApiError
  => super::super::super::super::Error
);
