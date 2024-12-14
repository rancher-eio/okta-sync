use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[remain::sorted]
pub struct Membership {
  pub downgrade_on_destroy: bool,
  pub role: Role,
  pub username: String,
}

impl Membership {
  pub const API_GROUP_VERSION: &str = "user.github.upbound.io/v1alpha1";
  pub const KIND: &str = "Membership";
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[remain::sorted]
pub enum Role {
  Admin,
  #[default]
  Member,
}
