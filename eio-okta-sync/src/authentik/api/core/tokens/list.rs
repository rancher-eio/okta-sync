use crate::authentik::HumanFriendlyDateTime;

use crate::authentik::prelude::*;
use authentik_client::apis::Error;
use authentik_client::apis::configuration::Configuration;
use authentik_client::apis::core_api::{CoreTokensListError, core_tokens_list};
use authentik_client::models::{IntentEnum, PaginatedTokenList, Token};

#[derive(Debug, Clone, PartialEq, Eq, clap::Parser, serde::Serialize, serde::Deserialize, bon::Builder)]
#[builder(on(String, into))]
#[command(about = "List (or search for) tokens")]
#[command(after_help = "API Reference @ https://api.goauthentik.io/reference/core-tokens-list/")]
#[remain::sorted]
pub struct CoreTokensList {
  #[arg(help_heading = "Query Parameters")]
  #[arg(long)]
  #[arg(value_name = "STRING")]
  description: Option<String>,
  #[arg(help_heading = "Query Parameters")]
  #[arg(long)]
  #[arg(value_name = "DATE-TIME")]
  #[serde(default)]
  #[serde(skip_serializing_if = "Option::is_none")]
  expires: Option<HumanFriendlyDateTime>,
  #[arg(action = clap::ArgAction::Set)]
  #[arg(help_heading = "Query Parameters")]
  #[arg(long)]
  #[arg(value_name = "BOOL")]
  #[serde(skip_serializing_if = "Option::is_none")]
  expiring: Option<bool>,
  #[arg(help_heading = "Query Parameters")]
  #[arg(long)]
  #[arg(value_name = "STRING")]
  #[serde(skip_serializing_if = "Option::is_none")]
  identifier: Option<String>,
  #[arg(help_heading = "Query Parameters")]
  #[arg(long)]
  #[arg(value_name = "INTENT")]
  #[serde(skip_serializing_if = "Option::is_none")]
  intent: Option<Intent>,
  #[arg(help_heading = "Query Parameters")]
  #[arg(long)]
  #[arg(value_name = "STRING")]
  #[serde(skip_serializing_if = "Option::is_none")]
  managed: Option<String>,
  #[arg(help_heading = "Query Parameters")]
  #[arg(long)]
  #[arg(value_name = "FIELD")]
  #[serde(skip_serializing_if = "Option::is_none")]
  ordering: Option<String>,
  #[arg(help_heading = "Query Parameters")]
  #[arg(long)]
  #[arg(value_name = "INTEGER")]
  #[serde(skip_serializing_if = "Option::is_none")]
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
  #[serde(skip_serializing_if = "Option::is_none")]
  search: Option<String>,
  #[arg(help_heading = "Query Parameters")]
  #[arg(long)]
  #[arg(value_name = "STRING")]
  #[serde(skip_serializing_if = "Option::is_none")]
  user_username: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, clap::ValueEnum)]
#[clap(rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[remain::sorted]
pub enum Intent {
  Api,
  AppPassword,
  Recovery,
  Verification,
}

impl From<Intent> for IntentEnum {
  fn from(value: Intent) -> Self {
    match value {
      Intent::Api => Self::Api,
      Intent::AppPassword => Self::AppPassword,
      Intent::Recovery => Self::Recovery,
      Intent::Verification => Self::Verification,
    }
  }
}

impl From<IntentEnum> for Intent {
  fn from(value: IntentEnum) -> Self {
    match value {
      IntentEnum::Api => Self::Api,
      IntentEnum::AppPassword => Self::AppPassword,
      IntentEnum::Recovery => Self::Recovery,
      IntentEnum::Verification => Self::Verification,
    }
  }
}

impl crate::authentik::traits::GetWithConfiguration for CoreTokensList {
  type Error = Error<CoreTokensListError>;
  type Value = PaginatedTokenList;

  async fn get_with_configuration(self, configuration: &Configuration) -> Result<Self::Value, Self::Error> {
    let Self {
      description,
      expires,
      expiring,
      identifier,
      intent,
      managed,
      ordering,
      page,
      page_size,
      search,
      user_username,
    } = self;

    core_tokens_list(
      configuration,
      description.map_as_str(),
      expires.map(Into::into),
      expiring,
      identifier.map_as_str(),
      intent.map(IntentEnum::from),
      managed.map_as_str(),
      ordering.map_as_str(),
      page,
      Some(page_size),
      search.map_as_str(),
      user_username.map_as_str(),
    )
    .await
  }
}

crate::authentik::macros::PageMut!(CoreTokensList);
crate::authentik::macros::Paginated!(PaginatedTokenList => Vec<Token>);
crate::authentik::macros::Default!(CoreTokensList as Builder);

crate::authentik::macros::defaults!(const DEFAULT_PAGE_SIZE: i32 = 100 as "100");

crate::authentik::macros::From!(
  Error<CoreTokensListError>
  => super::CoreTokensApiError
  => super::super::CoreApiError
  => super::super::super::ApiError
  => super::super::super::super::Error
);

#[cfg(test)]
mod tests {
  use super::*;

  crate::authentik::macros::test!(defaults for const DEFAULT_PAGE_SIZE as i32);
  crate::authentik::macros::test!(defaults for struct CoreTokensList);
}
