use indexmap::IndexMap;
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[remain::sorted]
pub struct TeamMembership {
  pub role: Role,
  #[serde(flatten)]
  pub team_id: TeamId,
  pub username: String,
}

impl TeamMembership {
  pub const API_GROUP_VERSION: &str = "team.github.upbound.io/v1alpha1";
  pub const KIND: &str = "TeamMembership";
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[remain::sorted]
pub enum TeamId {
  TeamId(String),
  TeamIdRef(TeamIdRef),
  TeamIdSelector(TeamIdSelector),
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[remain::sorted]
pub struct TeamIdRef {
  pub name: String,
  pub policy: Option<crate::crossplane::Policy>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[remain::sorted]
pub struct TeamIdSelector {
  pub match_controller_ref: bool,
  pub match_labels: IndexMap<String, Value>,
  pub policy: Option<crate::crossplane::Policy>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
#[remain::sorted]
pub enum Role {
  Maintainer,
  #[default]
  Member,
}

impl From<String> for TeamId {
  fn from(value: String) -> Self {
    Self::TeamId(value)
  }
}

impl From<TeamIdRef> for TeamId {
  fn from(value: TeamIdRef) -> Self {
    Self::TeamIdRef(value)
  }
}

impl From<TeamIdSelector> for TeamId {
  fn from(value: TeamIdSelector) -> Self {
    Self::TeamIdSelector(value)
  }
}
