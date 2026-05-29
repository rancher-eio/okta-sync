use crate::authentik::prelude::*;
use authentik_client::apis::Error;
use authentik_client::apis::configuration::Configuration;
use authentik_client::apis::core_api::{CoreGroupsListError, core_groups_list};
use authentik_client::models::{Group, PaginatedGroupList};

#[derive(Debug, Clone, PartialEq, Eq, clap::Parser, serde::Serialize, serde::Deserialize, bon::Builder)]
#[builder(on(String, into))]
#[command(about = "List (or search for) groups")]
#[command(after_help = "API Reference @ https://api.goauthentik.io/reference/core-groups-list/")]
#[remain::sorted]
pub struct CoreGroupsList {
  #[arg(help_heading = "Query Parameters")]
  #[arg(long)]
  #[arg(value_name = "STRING")]
  attributes: Option<String>,
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
  #[arg(action = clap::ArgAction::Set)]
  #[arg(help_heading = "Query Parameters")]
  #[arg(long)]
  #[arg(value_name = "BOOL")]
  is_superuser: Option<bool>,
  #[arg(help_heading = "Query Parameters")]
  #[arg(long)]
  #[arg(value_delimiter = ',')]
  #[arg(value_name = "INTEGER")]
  members_by_pk: Option<Vec<i32>>,
  #[arg(help_heading = "Query Parameters")]
  #[arg(long)]
  #[arg(value_delimiter = ',')]
  #[arg(value_name = "STRING")]
  members_by_username: Option<Vec<String>>,
  #[arg(help_heading = "Query Parameters")]
  #[arg(long)]
  #[arg(value_name = "STRING")]
  name: Option<String>,
  #[arg(help_heading = "Query Parameters")]
  #[arg(long)]
  #[arg(value_name = "STRING")]
  ordering: Option<String>,
  #[arg(help_heading = "Query Parameters")]
  #[arg(long)]
  #[arg(value_name = "INTEGER")]
  page: Option<i32>,
  #[arg(default_value = DEFAULT_PAGE_SIZE_STR)]
  #[arg(help_heading = "Query Parameters")]
  #[arg(long)]
  #[arg(value_name = "INTEGER")]
  #[builder(default = DEFAULT_PAGE_SIZE)]
  #[serde(default = "default_page_size")]
  page_size: i32,
  #[arg(help_heading = "Query Parameters")]
  #[arg(long)]
  #[arg(value_name = "STRING")]
  search: Option<String>,
}

impl crate::authentik::traits::GetWithConfiguration for CoreGroupsList {
  type Error = Error<CoreGroupsListError>;
  type Value = PaginatedGroupList;

  async fn get_with_configuration(self, configuration: &Configuration) -> Result<Self::Value, Self::Error> {
    let Self {
      attributes,
      include_children,
      include_inherited_roles,
      include_parents,
      include_users,
      is_superuser,
      members_by_pk,
      members_by_username,
      name,
      ordering,
      page,
      page_size,
      search,
    } = self;

    core_groups_list(
      configuration,
      attributes.map_as_str(),
      Some(include_children),
      Some(include_inherited_roles),
      Some(include_parents),
      Some(include_users),
      is_superuser,
      members_by_pk,
      members_by_username,
      name.map_as_str(),
      ordering.map_as_str(),
      page,
      Some(page_size),
      search.map_as_str(),
    )
    .await
  }
}

crate::authentik::macros::Default!(CoreGroupsList as Builder);
crate::authentik::macros::PageMut!(CoreGroupsList);
crate::authentik::macros::Paginated!(PaginatedGroupList => Vec<Group>);

crate::authentik::macros::defaults!(const DEFAULT_INCLUDE_CHILDREN: bool = false as "false");
crate::authentik::macros::defaults!(const DEFAULT_INCLUDE_INHERITED_ROLES: bool = false as "false");
crate::authentik::macros::defaults!(const DEFAULT_INCLUDE_PARENTS: bool = false as "false");
crate::authentik::macros::defaults!(const DEFAULT_INCLUDE_USERS: bool = true as "true");
crate::authentik::macros::defaults!(const DEFAULT_PAGE_SIZE: i32 = 100 as "100");

crate::authentik::macros::From!(
  Error<CoreGroupsListError>
  => super::CoreGroupsApiError
  => super::super::CoreApiError
  => super::super::super::ApiError
  => super::super::super::super::Error
);

#[cfg(test)]
mod tests {
  use super::*;

  crate::authentik::macros::test!(defaults for const DEFAULT_INCLUDE_CHILDREN as bool);
  crate::authentik::macros::test!(defaults for const DEFAULT_INCLUDE_INHERITED_ROLES as bool);
  crate::authentik::macros::test!(defaults for const DEFAULT_INCLUDE_PARENTS as bool);
  crate::authentik::macros::test!(defaults for const DEFAULT_INCLUDE_USERS as bool);
  crate::authentik::macros::test!(defaults for const DEFAULT_PAGE_SIZE as i32);
  crate::authentik::macros::test!(defaults for struct CoreGroupsList);
}
