use bounded_integer::BoundedU8;
use octocrab::Octocrab;

use crate::github::enterprise::models::EnterpriseTeamMembership;

#[derive(serde::Serialize, bon::Builder)]
#[builder(on(String, into), state_mod = builder)]
pub struct ListEnterpriseTeamMembershipsRequest<'client> {
  #[serde(skip)]
  client: &'client Octocrab,
  #[serde(skip)]
  enterprise: String,
  #[serde(skip)]
  team: String,
  #[builder(default = BoundedU8::const_new::<30>())]
  per_page: BoundedU8<0, 100>,
  #[builder(default = 1)]
  page: usize,
}

impl ListEnterpriseTeamMembershipsRequest<'_> {
  pub async fn send(self) -> octocrab::Result<octocrab::Page<EnterpriseTeamMembership>> {
    let route = format!(
      "/enterprises/{enterprise}/teams/{enterprise_team}/memberships",
      enterprise = self.enterprise,
      enterprise_team = self.team,
    );
    self.client.get(route, Some(&self)).await
  }
}

impl super::EnterpriseTeamMembershipsHandler<'_, '_, '_> {
  pub fn list(
    &self,
  ) -> ListEnterpriseTeamMembershipsRequestBuilder<'_, builder::SetTeam<builder::SetEnterprise<builder::SetClient>>> {
    ListEnterpriseTeamMembershipsRequest::builder()
      .client(self.client)
      .enterprise(self.enterprise)
      .team(self.team)
  }
}
