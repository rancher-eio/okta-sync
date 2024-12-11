#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[remain::sorted]
pub struct Team {
  pub create_default_maintainer: bool,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub description: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub ldap_dn: Option<String>,
  pub name: String,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub parent_team_id: Option<String>,
  pub privacy: Privacy,
}

impl Team {
  pub const API_GROUP_VERSION: &str = "team.github.upbound.io/v1alpha1";
  pub const KIND: &str = "Team";
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
#[remain::sorted]
pub enum Privacy {
  Closed,
  #[default]
  Secret,
}
