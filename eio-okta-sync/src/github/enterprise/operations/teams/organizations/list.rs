use bounded_integer::BoundedU8;
use octocrab::Octocrab;

use octocrab::models::orgs::Organization;

#[derive(serde::Serialize, bon::Builder)]
#[builder(on(String, into), state_mod = builder)]
pub struct ListOrganizationAssignmentsRequest<'client> {
  #[serde(skip)]
  client: &'client Octocrab,
  #[serde(skip)]
  enterprise: String,
  #[serde(skip)]
  team: String,
  #[builder(default = BoundedU8::const_new::<100>())]
  per_page: BoundedU8<0, 100>,
  #[builder(default = 1)]
  page: usize,
}

impl ListOrganizationAssignmentsRequest<'_> {
  #[tracing::instrument(skip(self), fields(enterprise = self.enterprise, team = self.team))]
  pub async fn send(self) -> octocrab::Result<octocrab::Page<Organization>> {
    let route = format!(
      "/enterprises/{enterprise}/teams/{enterprise_team}/organizations",
      enterprise = self.enterprise,
      enterprise_team = self.team,
    );
    self.client.get(route, Some(&self)).await
  }
}

impl super::EnterpriseTeamOrganizationsHandler<'_, '_, '_> {
  pub fn list(
    &self,
  ) -> ListOrganizationAssignmentsRequestBuilder<'_, builder::SetTeam<builder::SetEnterprise<builder::SetClient>>> {
    ListOrganizationAssignmentsRequest::builder()
      .client(self.client)
      .enterprise(self.enterprise)
      .team(self.team)
  }
}
