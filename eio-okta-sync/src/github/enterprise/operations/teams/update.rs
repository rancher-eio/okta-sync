use octocrab::Octocrab;

use self::update_enterprise_team_request_builder::*;
use crate::github::enterprise::models::{EnterpriseTeam, OrganizationSelectionType};

#[derive(serde::Serialize, bon::Builder)]
#[builder(on(String, into))]
pub struct UpdateEnterpriseTeamRequest<'client> {
  #[serde(skip)]
  client: &'client Octocrab,
  #[serde(skip)]
  enterprise: String,
  #[serde(skip)]
  team: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  description: Option<String>,
  #[builder(default)]
  organization_selection_type: OrganizationSelectionType,
  #[serde(skip_serializing_if = "Option::is_none")]
  group_id: Option<String>,
}

impl UpdateEnterpriseTeamRequest<'_> {
  pub async fn send(self) -> octocrab::Result<EnterpriseTeam> {
    let route = format!(
      "/enterprises/{enterprise}/teams/{team_slug}",
      enterprise = self.enterprise,
      team_slug = self.team
    );
    self.client.patch(route, Some(&self)).await
  }
}

impl super::EnterpriseTeamHandler<'_, '_, '_> {
  pub fn update(&self) -> UpdateEnterpriseTeamRequestBuilder<'_, SetTeam<SetEnterprise<SetClient>>> {
    UpdateEnterpriseTeamRequest::builder()
      .client(self.client)
      .enterprise(self.enterprise)
      .team(self.team)
  }
}
