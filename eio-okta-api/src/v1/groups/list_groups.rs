use eio_okta_data::v2024_07_0::management::components::{
  parameters::{QueryAfter, QueryLimit},
  schemas::Group,
};

use crate::traits::{IntoRequest, Pagination};

impl crate::Endpoint for ListGroups {
  const PATH: &'static str = "/api/v1/groups";
}

impl IntoRequest for ListGroups {
  type Body = ();
}

impl Pagination for ListGroups {
  type Item = Group;
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, clap::Args, validator::Validate)]
#[serde(rename_all = "camelCase")]
#[group(skip)]
#[remain::sorted]
pub struct ListGroups {
  #[command(flatten)]
  #[serde(flatten)]
  pub after: QueryAfter,
  #[arg(long)]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub expand: Option<Expand>,
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
  pub sort_order: Option<SortOrder>,
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize, clap::ValueEnum)]
#[serde(rename_all = "lowercase")]
pub enum Expand {
  App,
  Stats,
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize, clap::ValueEnum)]
#[serde(rename_all = "lowercase")]
pub enum SortOrder {
  Asc,
  Desc,
}

impl AsRef<()> for ListGroups {
  fn as_ref(&self) -> &() {
    &()
  }
}
