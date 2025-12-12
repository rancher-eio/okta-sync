use chrono::{DateTime, Utc};
use iri_string::{template::UriTemplateString, types::UriAbsoluteString};
use monostate::MustBe;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
#[remain::sorted]
pub struct EnterpriseTeam {
  pub created_at: DateTime<Utc>,
  pub description: String,
  pub group_id: Option<String>,
  pub group_name: Option<String>,
  pub html_url: UriAbsoluteString,
  pub id: i64,
  pub members_url: UriTemplateString,
  pub name: String,
  pub organization_selection_type: OrganizationSelectionType,
  pub slug: String,
  pub updated_at: DateTime<Utc>,
  pub url: UriAbsoluteString,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
#[remain::sorted]
pub enum OrganizationSelectionType {
  All,
  #[default]
  Disabled,
  Selected,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
#[remain::sorted]
pub struct EnterpriseTeamMembership {
  pub avatar_url: UriAbsoluteString,
  pub events_url: UriTemplateString,
  pub followers_url: UriAbsoluteString,
  pub following_url: UriTemplateString,
  pub gists_url: UriTemplateString,
  pub gravatar_id: Option<String>,
  pub html_url: UriAbsoluteString,
  pub id: i64,
  pub login: String,
  pub node_id: Option<String>,
  pub organizations_url: UriAbsoluteString,
  pub r#type: String,
  pub received_events_url: UriAbsoluteString,
  pub repos_url: UriAbsoluteString,
  pub site_admin: bool,
  pub starred_url: UriTemplateString,
  pub subscriptions_url: UriAbsoluteString,
  pub url: UriAbsoluteString,
  pub user_view_type: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[remain::sorted]
pub struct ListCostCentersResponse {
  pub cost_centers: Vec<CostCenter>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
#[remain::sorted]
pub struct CostCenter {
  pub azure_subscription: Option<Uuid>,
  pub id: Uuid,
  pub name: String,
  pub resources: Vec<Resource>,
  pub state: Option<CostCenterState>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
#[remain::sorted]
pub enum CostCenterState {
  Active,
  Deleted,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
#[remain::sorted]
pub struct Resource {
  pub name: String,
  pub r#type: String,
}

impl From<EnterpriseTeamMembership> for Resource {
  fn from(
    EnterpriseTeamMembership {
      login: name, r#type, ..
    }: EnterpriseTeamMembership,
  ) -> Self {
    Self { name, r#type }
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
#[remain::sorted]
pub struct AddedResourcesResponse {
  pub message: String,
  pub reassigned_resources: Vec<ReassignedResource>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
#[remain::sorted]
pub struct ReassignedResource {
  pub name: String,
  pub previous_cost_center: String,
  pub resource_type: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
#[remain::sorted]
pub struct RemoveResourcesResponse {
  pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[remain::sorted]
pub struct DeleteCostCenterResponse {
  pub cost_center_state: MustBe!("CostCenterArchived"),
  pub id: Uuid,
  pub message: String,
  pub name: String,
}
