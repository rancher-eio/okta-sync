use clap::Args;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Args, Serialize, Deserialize, Validate)]
#[group(skip)]
#[remain::sorted]
pub struct Pagination {
  #[arg(long = "page-after")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub after: Option<String>,
  #[arg(long = "page-before")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub before: Option<String>,
  #[arg(long = "page-limit")]
  #[serde(skip_serializing_if = "Option::is_none")]
  #[validate(range(min = 0, max = 200))]
  pub limit: Option<i32>,
}
