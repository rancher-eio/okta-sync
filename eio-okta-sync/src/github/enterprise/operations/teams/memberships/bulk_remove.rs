use octocrab::Octocrab;

use crate::github::enterprise::models::EnterpriseTeamMembership;

#[derive(serde::Serialize, bon::Builder)]
#[builder(on(String, into), state_mod = builder)]
pub struct BulkRemoveEnterpriseTeamMembershipsRequest<'client> {
  #[serde(skip)]
  client: &'client Octocrab,
  #[serde(skip)]
  enterprise: String,
  #[serde(skip)]
  team: String,
  usernames: Vec<String>,
}

impl BulkRemoveEnterpriseTeamMembershipsRequest<'_> {
  #[tracing::instrument(skip(self), fields(enterprise = self.enterprise, team = self.team, usernames = self.usernames.len()))]
  pub async fn send(self) -> octocrab::Result<Vec<EnterpriseTeamMembership>> {
    let route = format!(
      "/enterprises/{enterprise}/teams/{enterprise_team}/memberships/remove",
      enterprise = self.enterprise,
      enterprise_team = self.team
    );
    self.client.post(route, Some(&self)).await
  }
}

impl super::EnterpriseTeamMembershipsHandler<'_, '_, '_> {
  pub fn bulk_remove(
    &self,
  ) -> BulkRemoveEnterpriseTeamMembershipsRequestBuilder<'_, builder::SetTeam<builder::SetEnterprise<builder::SetClient>>>
  {
    BulkRemoveEnterpriseTeamMembershipsRequest::builder()
      .client(self.client)
      .enterprise(self.enterprise)
      .team(self.team)
  }
}
