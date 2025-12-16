use octocrab::Octocrab;

use crate::github::enterprise::models::EnterpriseTeamMembership;

#[derive(bon::Builder)]
#[builder(on(String, into), state_mod = builder)]
pub struct AddEnterpriseTeamMembershipRequest<'client> {
  client: &'client Octocrab,
  enterprise: String,
  team: String,
  username: String,
}

impl AddEnterpriseTeamMembershipRequest<'_> {
  #[tracing::instrument(skip(self), fields(enterprise = self.enterprise, team = self.team, username = self.username))]
  pub async fn send(self) -> octocrab::Result<EnterpriseTeamMembership> {
    let route = format!(
      "/enterprises/{enterprise}/teams/{enterprise_team}/memberships/{username}",
      enterprise = self.enterprise,
      enterprise_team = self.team,
      username = self.username,
    );
    self.client.put(route, Option::<&()>::None).await
  }
}

impl super::EnterpriseTeamMembershipsHandler<'_, '_, '_> {
  pub fn add(&self, username: impl Into<String>) -> AddEnterpriseTeamMembershipRequest<'_> {
    AddEnterpriseTeamMembershipRequest::builder()
      .client(self.client)
      .enterprise(self.enterprise)
      .team(self.team)
      .username(username)
      .build()
  }
}

impl super::EnterpriseTeamMembershipHandler<'_, '_, '_, '_> {
  pub fn add(&self) -> AddEnterpriseTeamMembershipRequest<'_> {
    AddEnterpriseTeamMembershipRequest::builder()
      .client(self.client)
      .enterprise(self.enterprise)
      .team(self.team)
      .username(self.username)
      .build()
  }
}
