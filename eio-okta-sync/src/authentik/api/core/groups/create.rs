use std::collections::HashMap;

use authentik_client::apis::Error;
use authentik_client::apis::configuration::Configuration;
use authentik_client::apis::core_api::{CoreGroupsCreateError, core_groups_create};
use authentik_client::models::{Group, GroupRequest};
use serde_json::Value;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, clap::Parser, serde::Serialize, serde::Deserialize, bon::Builder)]
#[builder(on(String, into))]
#[command(about = "Create a group")]
#[command(after_help = "API Reference @ https://api.goauthentik.io/reference/core-groups-create/")]
#[remain::sorted]
pub struct CoreGroupsCreate {
  #[arg(help_heading = "Query Parameters")]
  #[arg(long)]
  #[arg(value_name = "JSON-OBJECT")]
  #[arg(value_parser = parse_json::<Option<HashMap<String, Value>>>)]
  #[builder(with = FromIterator::from_iter)]
  #[serde(default, skip_serializing_if = "Option::is_none")]
  attributes: Option<HashMap<String, Value>>,
  #[arg(action = clap::ArgAction::Set)]
  #[arg(default_value = DEFAULT_IS_SUPERUSER_STR)]
  #[arg(help_heading = "Query Parameters")]
  #[arg(long)]
  #[arg(value_name = "BOOLEAN")]
  #[builder(default = DEFAULT_IS_SUPERUSER)]
  #[serde(default = "default_is_superuser")]
  is_superuser: bool,
  #[arg(help_heading = "Query Parameters")]
  #[arg(long)]
  #[arg(value_name = "STRING")]
  name: String,
  #[arg(help_heading = "Query Parameters")]
  #[arg(long)]
  #[arg(value_delimiter = ',')]
  #[builder(with = FromIterator::from_iter)]
  #[serde(default, skip_serializing_if = "Option::is_none")]
  parents: Option<Vec<Uuid>>,
  #[arg(help_heading = "Query Parameters")]
  #[arg(long)]
  #[arg(value_delimiter = ',')]
  #[builder(with = FromIterator::from_iter)]
  #[serde(default, skip_serializing_if = "Option::is_none")]
  roles: Option<Vec<Uuid>>,
  #[arg(help_heading = "Query Parameters")]
  #[arg(long)]
  #[arg(value_delimiter = ',')]
  #[arg(value_name = "INTEGER")]
  #[builder(with = FromIterator::from_iter)]
  #[serde(default, skip_serializing_if = "Option::is_none")]
  users: Option<Vec<i32>>,
}

impl From<CoreGroupsCreate> for GroupRequest {
  fn from(value: CoreGroupsCreate) -> Self {
    let CoreGroupsCreate {
      name,
      is_superuser,
      parents,
      users,
      attributes,
      roles,
    } = value;

    Self {
      name,
      is_superuser: Some(is_superuser),
      parents,
      users,
      attributes,
      roles,
    }
  }
}

fn parse_json<T: serde::de::DeserializeOwned>(s: &str) -> Result<T, serde_json::Error> {
  serde_json::from_str(s)
}

impl crate::authentik::traits::GetWithConfiguration for CoreGroupsCreate {
  type Error = Error<CoreGroupsCreateError>;
  type Value = Group;

  async fn get_with_configuration(self, configuration: &Configuration) -> Result<Self::Value, Self::Error> {
    core_groups_create(configuration, self.into()).await
  }
}

crate::authentik::macros::defaults!(const DEFAULT_IS_SUPERUSER: bool = false as "false");

crate::authentik::macros::RunAsync!(CoreGroupsCreate as Wrapper<GetWithConfiguration>);

crate::authentik::macros::From!(
  Error<CoreGroupsCreateError>
  => super::CoreGroupsApiError
  => super::super::CoreApiError
  => super::super::super::ApiError
  => super::super::super::super::Error
);

#[cfg(test)]
mod tests {
  use super::*;

  crate::authentik::macros::test!(defaults for const DEFAULT_IS_SUPERUSER as bool);
}
