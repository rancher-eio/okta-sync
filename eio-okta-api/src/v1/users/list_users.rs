use eio_okta_data::v2024_07_0::management::components::{
  parameters::{QueryAfter, QueryLimit},
  schemas::User,
};

use crate::{
  traits::{IntoRequest, Pagination},
  Endpoint, MapInto, QueryString,
};

impl Endpoint for ListUsers {
  const PATH: &'static str = "/api/v1/users";

  fn query(&self) -> Result<QueryString, crate::traits::endpoint::Error> {
    QueryString::simple(self).map_into()
  }
}

impl IntoRequest for ListUsers {
  type Body = ();
}

impl Pagination for ListUsers {
  type Item = User;
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, clap::Args, validator::Validate)]
#[serde(rename_all = "camelCase")]
#[group(skip)]
#[remain::sorted]
pub struct ListUsers {
  #[command(flatten)]
  #[serde(flatten)]
  pub after: QueryAfter,
  #[command(flatten)]
  #[serde(flatten)]
  pub filter: crate::qol::Filter,
  #[command(flatten)]
  #[serde(flatten)]
  pub limit: QueryLimit,
  #[arg(long)]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub q: Option<String>,
  #[arg(long)]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub search: Option<String>,
  #[arg(long)]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub sort_by: Option<String>,
  #[arg(long)]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub sort_order: Option<String>,
}

impl AsRef<()> for ListUsers {
  fn as_ref(&self) -> &() {
    &()
  }
}
