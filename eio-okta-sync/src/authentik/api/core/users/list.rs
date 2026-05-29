use crate::authentik::HumanFriendlyDateTime;
use crate::authentik::prelude::*;
use authentik_client::apis::Error;
use authentik_client::apis::configuration::Configuration;
use authentik_client::apis::core_api::{CoreUsersListError, core_users_list};
use authentik_client::models::{PaginatedUserList, User, UserTypeEnum};
use email_address::EmailAddress;
use uuid::Uuid;

// passing dozens of **positional** arguments to a function feels pretty gross, especially when most of those are likely to be None.
// core_users_list(&configuration, None, None, None, None, Some("email@domain.tld"), None, None, ...) is 🤢
// CoreUsersList::build().email("email@domain.tld").build() feels... better, hopefully.

#[derive(Debug, Clone, PartialEq, Eq, clap::Parser, serde::Serialize, serde::Deserialize, bon::Builder)]
#[builder(on(String, into))]
#[command(about = "List (or search for) users")]
#[command(after_help = "API Reference @ https://api.goauthentik.io/reference/core-users-list/")]
#[remain::sorted]
pub struct CoreUsersList {
  #[arg(help_heading = "Query Parameters")]
  #[arg(long)]
  #[arg(value_name = "STRING")]
  attributes: Option<String>,
  #[arg(help_heading = "Query Parameters")]
  #[arg(long)]
  #[arg(value_name = "DATE-TIME")]
  #[serde(default, skip_serializing_if = "Option::is_none")]
  date_joined: Option<HumanFriendlyDateTime>,
  #[arg(help_heading = "Query Parameters")]
  #[arg(long)]
  #[arg(value_name = "DATE-TIME")]
  #[serde(default, skip_serializing_if = "Option::is_none")]
  date_joined_gt: Option<HumanFriendlyDateTime>,
  #[arg(help_heading = "Query Parameters")]
  #[arg(long)]
  #[arg(value_name = "DATE-TIME")]
  #[serde(default, skip_serializing_if = "Option::is_none")]
  date_joined_lt: Option<HumanFriendlyDateTime>,
  #[arg(help_heading = "Query Parameters")]
  #[arg(long)]
  #[arg(value_name = "ADDRESS")]
  #[serde(default, skip_serializing_if = "Option::is_none")]
  email: Option<EmailAddress>,
  #[arg(help_heading = "Query Parameters")]
  #[arg(long)]
  #[arg(value_delimiter = ',')]
  #[arg(value_name = "STRING")]
  #[builder(with = FromIterator::from_iter)]
  #[serde(default, skip_serializing_if = "Option::is_none")]
  groups_by_name: Option<Vec<String>>,
  #[arg(help_heading = "Query Parameters")]
  #[arg(long)]
  #[arg(value_delimiter = ',')]
  #[arg(value_name = "UUID")]
  #[builder(with = FromIterator::from_iter)]
  #[serde(default, skip_serializing_if = "Option::is_none")]
  groups_by_pk: Option<Vec<Uuid>>,
  #[arg(action = clap::ArgAction::Set)]
  #[arg(default_value = DEFAULT_INCLUDE_GROUPS_STR)]
  #[arg(help_heading = "Query Parameters")]
  #[arg(long)]
  #[arg(value_name = "BOOL")]
  #[builder(default = DEFAULT_INCLUDE_GROUPS)]
  #[serde(default = "default_include_groups")]
  include_groups: bool,
  #[arg(action = clap::ArgAction::Set)]
  #[arg(default_value = DEFAULT_INCLUDE_ROLES_STR)]
  #[arg(help_heading = "Query Parameters")]
  #[arg(long)]
  #[arg(value_name = "BOOL")]
  #[builder(default = DEFAULT_INCLUDE_ROLES)]
  #[serde(default = "default_include_roles")]
  include_roles: bool,
  #[arg(action = clap::ArgAction::Set)]
  #[arg(help_heading = "Query Parameters")]
  #[arg(long)]
  #[arg(value_name = "BOOL")]
  #[serde(default, skip_serializing_if = "Option::is_none")]
  is_active: Option<bool>,
  #[arg(action = clap::ArgAction::Set)]
  #[arg(help_heading = "Query Parameters")]
  #[arg(long)]
  #[arg(value_name = "BOOL")]
  #[serde(default, skip_serializing_if = "Option::is_none")]
  is_superuser: Option<bool>,
  #[arg(help_heading = "Query Parameters")]
  #[arg(long)]
  #[arg(value_name = "DATE-TIME")]
  #[serde(default, skip_serializing_if = "Option::is_none")]
  last_login: Option<HumanFriendlyDateTime>,
  #[arg(help_heading = "Query Parameters")]
  #[arg(long)]
  #[arg(value_name = "DATE-TIME")]
  #[serde(default, skip_serializing_if = "Option::is_none")]
  last_login_gt: Option<HumanFriendlyDateTime>,
  #[arg(action = clap::ArgAction::Set)]
  #[arg(help_heading = "Query Parameters")]
  #[arg(long)]
  #[arg(value_name = "BOOL")]
  #[serde(default, skip_serializing_if = "Option::is_none")]
  last_login_isnull: Option<bool>,
  #[arg(help_heading = "Query Parameters")]
  #[arg(long)]
  #[arg(value_name = "DATE-TIME")]
  #[serde(default, skip_serializing_if = "Option::is_none")]
  last_login_lt: Option<HumanFriendlyDateTime>,
  #[arg(help_heading = "Query Parameters")]
  #[arg(long)]
  #[arg(value_name = "DATE-TIME")]
  #[serde(default, skip_serializing_if = "Option::is_none")]
  last_updated: Option<HumanFriendlyDateTime>,
  #[arg(help_heading = "Query Parameters")]
  #[arg(long)]
  #[arg(value_name = "DATE-TIME")]
  #[serde(default, skip_serializing_if = "Option::is_none")]
  last_updated_gt: Option<HumanFriendlyDateTime>,
  #[arg(help_heading = "Query Parameters")]
  #[arg(long)]
  #[arg(value_name = "DATE-TIME")]
  #[serde(default, skip_serializing_if = "Option::is_none")]
  last_updated_lt: Option<HumanFriendlyDateTime>,
  #[arg(help_heading = "Query Parameters")]
  #[arg(long)]
  #[arg(value_name = "STRING")]
  name: Option<String>,
  #[arg(help_heading = "Query Parameters")]
  #[arg(long)]
  #[arg(value_name = "FIELD")]
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
  path: Option<String>,
  #[arg(help_heading = "Query Parameters")]
  #[arg(long)]
  #[arg(value_name = "STRING")]
  path_startswith: Option<String>,
  #[arg(help_heading = "Query Parameters")]
  #[arg(long)]
  #[arg(value_name = "STRING")]
  roles_by_name: Option<Vec<String>>,
  #[arg(help_heading = "Query Parameters")]
  #[arg(long)]
  #[arg(value_delimiter = ',')]
  #[arg(value_name = "UUID")]
  roles_by_pk: Option<Vec<Uuid>>,
  #[arg(help_heading = "Query Parameters")]
  #[arg(long)]
  #[arg(value_name = "STRING")]
  search: Option<String>,
  #[arg(help_heading = "Query Parameters")]
  #[arg(long = "type")]
  #[arg(value_delimiter = ',')]
  #[arg(value_name = "USER-TYPE")]
  #[builder(name = r#type, with = FromIterator::from_iter)]
  #[serde(default, rename = "type", skip_serializing_if = "Option::is_none")]
  type_: Option<Vec<UserType>>,
  #[arg(help_heading = "Query Parameters")]
  #[arg(long)]
  #[arg(value_name = "STRING")]
  username: Option<String>,
  #[arg(help_heading = "Query Parameters")]
  #[arg(long)]
  #[arg(value_name = "UUID")]
  uuid: Option<Uuid>,
}

impl crate::authentik::traits::GetWithConfiguration for CoreUsersList {
  type Error = Error<CoreUsersListError>;
  type Value = PaginatedUserList;

  async fn get_with_configuration(self, configuration: &Configuration) -> Result<Self::Value, Self::Error> {
    let Self {
      attributes,
      date_joined,
      date_joined_gt,
      date_joined_lt,
      email,
      groups_by_name,
      groups_by_pk,
      include_groups,
      include_roles,
      is_active,
      is_superuser,
      last_login,
      last_login_gt,
      last_login_isnull,
      last_login_lt,
      last_updated,
      last_updated_gt,
      last_updated_lt,
      name,
      ordering,
      page,
      page_size,
      path,
      path_startswith,
      roles_by_name,
      roles_by_pk,
      search,
      type_,
      username,
      uuid,
    } = self;

    core_users_list(
      configuration,
      attributes.map_as_str(),
      date_joined.map(Into::into),
      date_joined_gt.map(Into::into),
      date_joined_lt.map(Into::into),
      email.map_as_str(),
      groups_by_name,
      groups_by_pk,
      Some(include_groups),
      Some(include_roles),
      is_active,
      is_superuser,
      last_login.map(Into::into),
      last_login_gt.map(Into::into),
      last_login_isnull,
      last_login_lt.map(Into::into),
      last_updated.map(Into::into),
      last_updated_gt.map(Into::into),
      last_updated_lt.map(Into::into),
      name.map_as_str(),
      ordering.map_as_str(),
      page,
      Some(page_size),
      path.map_as_str(),
      path_startswith.map_as_str(),
      roles_by_name,
      roles_by_pk,
      search.map_as_str(),
      type_.map(|values| values.into_iter().map(UserTypeEnum::from).collect()),
      username.map_as_str(),
      uuid.map(|uuid| uuid.to_string()).map_as_str(),
    )
    .await
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, clap::ValueEnum)]
#[clap(rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum UserType {
  Internal,
  External,
  ServiceAccount,
  InternalServiceAccount,
}

impl From<UserType> for UserTypeEnum {
  fn from(value: UserType) -> Self {
    match value {
      UserType::External => Self::External,
      UserType::Internal => Self::Internal,
      UserType::InternalServiceAccount => Self::InternalServiceAccount,
      UserType::ServiceAccount => Self::ServiceAccount,
    }
  }
}

impl From<UserTypeEnum> for UserType {
  fn from(value: UserTypeEnum) -> Self {
    match value {
      UserTypeEnum::External => Self::External,
      UserTypeEnum::Internal => Self::Internal,
      UserTypeEnum::InternalServiceAccount => Self::InternalServiceAccount,
      UserTypeEnum::ServiceAccount => Self::ServiceAccount,
    }
  }
}

crate::authentik::macros::Default!(CoreUsersList as Builder);
crate::authentik::macros::PageMut!(CoreUsersList);
crate::authentik::macros::Paginated!(PaginatedUserList => Vec<User>);

crate::authentik::macros::defaults!(const DEFAULT_INCLUDE_GROUPS: bool = true as "true");
crate::authentik::macros::defaults!(const DEFAULT_INCLUDE_ROLES: bool = true as "true");
crate::authentik::macros::defaults!(const DEFAULT_PAGE_SIZE: i32 = 100 as "100");

crate::authentik::macros::From!(
  Error<CoreUsersListError>
  => super::CoreUsersApiError
  => super::super::CoreApiError
  => super::super::super::ApiError
  => super::super::super::super::Error
);

#[cfg(test)]
mod tests {
  use super::*;

  crate::authentik::macros::test!(defaults for const DEFAULT_INCLUDE_GROUPS as bool);
  crate::authentik::macros::test!(defaults for const DEFAULT_INCLUDE_ROLES as bool);
  crate::authentik::macros::test!(defaults for const DEFAULT_PAGE_SIZE as i32);
  crate::authentik::macros::test!(defaults for struct CoreUsersList);
}
