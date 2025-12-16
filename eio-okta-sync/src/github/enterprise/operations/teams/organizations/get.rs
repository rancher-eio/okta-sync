use octocrab::Octocrab;

use octocrab::models::orgs::Organization;

#[derive(bon::Builder)]
#[builder(on(String, into), state_mod = builder)]
pub struct GetOrganizationAssignmentRequest<'client> {
  client: &'client Octocrab,
  enterprise: String,
  team: String,
  org: String,
}

impl GetOrganizationAssignmentRequest<'_> {
  #[tracing::instrument(skip(self), fields(enterprise = self.enterprise, team = self.team, org = self.org))]
  pub async fn send(self) -> octocrab::Result<Organization> {
    let route = format!(
      "/enterprises/{enterprise}/teams/{enterprise_team}/organizations/{org}",
      enterprise = self.enterprise,
      enterprise_team = self.team,
      org = self.org,
    );
    self.client.get(route, Option::<&()>::None).await
  }
}

impl super::EnterpriseTeamOrganizationsHandler<'_, '_, '_> {
  pub fn get(&self, org: impl Into<String>) -> GetOrganizationAssignmentRequest<'_> {
    GetOrganizationAssignmentRequest::builder()
      .client(self.client)
      .enterprise(self.enterprise)
      .team(self.team)
      .org(org)
      .build()
  }
}

impl super::EnterpriseTeamOrganizationHandler<'_, '_, '_, '_> {
  pub fn get(&self) -> GetOrganizationAssignmentRequest<'_> {
    GetOrganizationAssignmentRequest::builder()
      .client(self.client)
      .enterprise(self.enterprise)
      .team(self.team)
      .org(self.org)
      .build()
  }
}
