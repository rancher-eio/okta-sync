use std::collections::HashSet;

use octocrab::{Octocrab, models::orgs::Organization};

#[derive(serde::Serialize, bon::Builder)]
#[builder(on(String, into), state_mod = builder)]
pub struct AddOrganizationAssignmentsRequest<'client> {
  #[serde(skip)]
  client: &'client Octocrab,
  #[serde(skip)]
  enterprise: String,
  #[serde(skip)]
  team: String,
  #[builder(with = <_>::from_iter)]
  #[serde(rename = "organization_slugs")]
  orgs: HashSet<String>,
}

impl AddOrganizationAssignmentsRequest<'_> {
  #[tracing::instrument(skip(self), fields(enterprise = self.enterprise, team = self.team, orgs = self.orgs.len()))]
  pub async fn send(self) -> octocrab::Result<Vec<Organization>> {
    let route = format!(
      "/enterprises/{enterprise}/teams/{enterprise_team}/organizations/add",
      enterprise = self.enterprise,
      enterprise_team = self.team
    );
    self.client.post(route, Some(&self)).await
  }
}

impl super::EnterpriseTeamOrganizationsHandler<'_, '_, '_> {
  pub fn bulk_add(
    &self,
  ) -> AddOrganizationAssignmentsRequestBuilder<'_, builder::SetTeam<builder::SetEnterprise<builder::SetClient>>> {
    AddOrganizationAssignmentsRequest::builder()
      .client(self.client)
      .enterprise(self.enterprise)
      .team(self.team)
  }
}
