use authentik_client::apis::Error;
use authentik_client::apis::configuration::Configuration;
use authentik_client::apis::core_api::{CoreGroupsAddUserCreateError, core_groups_add_user_create};
use authentik_client::models::UserAccountRequest;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, clap::Parser, serde::Serialize, serde::Deserialize, bon::Builder)]
#[command(about = "Add user to group")]
#[command(after_help = "API Reference @ https://api.goauthentik.io/reference/core-groups-add-user-create/")]
#[remain::sorted]
pub struct CoreGroupsAddUserCreate {
  #[arg(help_heading = "Query Parameters")]
  #[arg(long)]
  #[arg(value_name = "UUID")]
  group_uuid: Uuid,
  #[arg(help_heading = "Query Parameters")]
  #[arg(long, visible_alias = "user-id")]
  #[arg(value_name = "INTEGER")]
  pk: i32,
}

impl crate::authentik::traits::GetWithConfiguration for CoreGroupsAddUserCreate {
  type Error = Error<CoreGroupsAddUserCreateError>;
  type Value = ();

  async fn get_with_configuration(self, configuration: &Configuration) -> Result<Self::Value, Self::Error> {
    let Self { group_uuid, pk } = self;

    let user_account_request = UserAccountRequest::new(pk);

    core_groups_add_user_create(configuration, &group_uuid.to_string(), user_account_request).await
  }
}

crate::authentik::macros::RunAsync!(CoreGroupsAddUserCreate as Wrapper<GetWithConfiguration>);

crate::authentik::macros::From!(
  Error<CoreGroupsAddUserCreateError>
  => super::CoreGroupsApiError
  => super::super::CoreApiError
  => super::super::super::ApiError
  => super::super::super::super::Error
);
