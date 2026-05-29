use authentik_client::apis::Error;
use authentik_client::apis::configuration::Configuration;
use authentik_client::apis::core_api::{CoreGroupsRetrieveError, core_groups_retrieve};
use authentik_client::models::Group;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, clap::Parser, serde::Serialize, serde::Deserialize, bon::Builder)]
#[command(about = "Get information about a specific group")]
#[command(after_help = "API Reference @ https://api.goauthentik.io/reference/core-groups-retrieve/")]
#[remain::sorted]
pub struct CoreGroupsRetrieve {
  #[arg(help_heading = "Query Parameters")]
  #[arg(long)]
  #[arg(value_name = "UUID")]
  group_uuid: Uuid,
  #[arg(action = clap::ArgAction::Set)]
  #[arg(default_value = DEFAULT_INCLUDE_CHILDREN_STR)]
  #[arg(help_heading = "Query Parameters")]
  #[arg(long)]
  #[arg(value_name = "BOOL")]
  #[builder(default = DEFAULT_INCLUDE_CHILDREN)]
  #[serde(default = "default_include_children")]
  include_children: bool,
  #[arg(action = clap::ArgAction::Set)]
  #[arg(default_value = DEFAULT_INCLUDE_INHERITED_ROLES_STR)]
  #[arg(help_heading = "Query Parameters")]
  #[arg(long)]
  #[arg(value_name = "BOOL")]
  #[builder(default = DEFAULT_INCLUDE_INHERITED_ROLES)]
  #[serde(default = "default_include_inherited_roles")]
  include_inherited_roles: bool,
  #[arg(action = clap::ArgAction::Set)]
  #[arg(default_value = DEFAULT_INCLUDE_PARENTS_STR)]
  #[arg(help_heading = "Query Parameters")]
  #[arg(long)]
  #[arg(value_name = "BOOL")]
  #[builder(default = DEFAULT_INCLUDE_PARENTS)]
  #[serde(default = "default_include_parents")]
  include_parents: bool,
  #[arg(action = clap::ArgAction::Set)]
  #[arg(default_value = DEFAULT_INCLUDE_USERS_STR)]
  #[arg(help_heading = "Query Parameters")]
  #[arg(long)]
  #[arg(value_name = "BOOL")]
  #[builder(default = DEFAULT_INCLUDE_USERS)]
  #[serde(default = "default_include_users")]
  include_users: bool,
}

impl crate::authentik::traits::GetWithConfiguration for CoreGroupsRetrieve {
  type Error = Error<CoreGroupsRetrieveError>;
  type Value = Group;

  async fn get_with_configuration(self, configuration: &Configuration) -> Result<Self::Value, Self::Error> {
    let Self {
      group_uuid,
      include_children,
      include_inherited_roles,
      include_parents,
      include_users,
    } = self;

    core_groups_retrieve(
      configuration,
      &group_uuid.to_string(),
      Some(include_children),
      Some(include_inherited_roles),
      Some(include_parents),
      Some(include_users),
    )
    .await
  }
}

crate::authentik::macros::defaults!(const DEFAULT_INCLUDE_CHILDREN: bool = false as "false");
crate::authentik::macros::defaults!(const DEFAULT_INCLUDE_INHERITED_ROLES: bool = false as "false");
crate::authentik::macros::defaults!(const DEFAULT_INCLUDE_PARENTS: bool = false as "false");
crate::authentik::macros::defaults!(const DEFAULT_INCLUDE_USERS: bool = true as "true");

#[cfg(test)]
mod tests {
  use super::*;

  crate::authentik::macros::test!(defaults for const DEFAULT_INCLUDE_CHILDREN as bool);
  crate::authentik::macros::test!(defaults for const DEFAULT_INCLUDE_INHERITED_ROLES as bool);
  crate::authentik::macros::test!(defaults for const DEFAULT_INCLUDE_PARENTS as bool);
  crate::authentik::macros::test!(defaults for const DEFAULT_INCLUDE_USERS as bool);
}

crate::authentik::macros::RunAsync!(CoreGroupsRetrieve as Wrapper<GetWithConfiguration>);

crate::authentik::macros::From!(
  Error<CoreGroupsRetrieveError>
  => super::CoreGroupsApiError
  => super::super::CoreApiError
  => super::super::super::ApiError
  => super::super::super::super::Error
);
