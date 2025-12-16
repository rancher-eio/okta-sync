use octocrab::Octocrab;

use crate::github::enterprise::models::{EnterpriseTeam, OrganizationSelectionType};

#[derive(serde::Serialize, bon::Builder)]
#[builder(on(String, into), state_mod = builder)]
pub struct CreateEnterpriseTeamRequest<'client> {
  #[serde(skip)]
  client: &'client Octocrab,
  #[serde(skip)]
  enterprise: String,
  name: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  description: Option<String>,
  #[builder(default)]
  organization_selection_type: OrganizationSelectionType,
  #[serde(skip_serializing_if = "Option::is_none")]
  group_id: Option<String>,
}

impl CreateEnterpriseTeamRequest<'_> {
  #[tracing::instrument(skip(self), fields(enterprise = self.enterprise, name = self.name))]
  pub async fn send(self) -> octocrab::Result<EnterpriseTeam> {
    let route = format!("/enterprises/{enterprise}/teams", enterprise = self.enterprise);
    self.client.post(route, Some(&self)).await
  }
}

impl super::EnterpriseTeamsHandler<'_, '_> {
  pub fn create(
    &self,
    name: impl Into<String>,
  ) -> CreateEnterpriseTeamRequestBuilder<'_, builder::SetName<builder::SetEnterprise<builder::SetClient>>> {
    CreateEnterpriseTeamRequest::builder()
      .client(self.client)
      .enterprise(self.enterprise)
      .name(name)
  }
}

impl super::EnterpriseTeamHandler<'_, '_, '_> {
  pub fn create(
    &self,
  ) -> CreateEnterpriseTeamRequestBuilder<'_, builder::SetName<builder::SetEnterprise<builder::SetClient>>> {
    CreateEnterpriseTeamRequest::builder()
      .client(self.client)
      .enterprise(self.enterprise)
      .name(self.team)
  }
}
