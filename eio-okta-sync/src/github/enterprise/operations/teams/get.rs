use octocrab::Octocrab;

use crate::github::enterprise::models::EnterpriseTeam;

#[derive(bon::Builder)]
#[builder(on(String, into), state_mod = builder)]
pub struct GetEnterpriseTeamRequest<'client> {
  client: &'client Octocrab,
  enterprise: String,
  team: String,
}

impl GetEnterpriseTeamRequest<'_> {
  #[tracing::instrument(skip(self), fields(enterprise = self.enterprise, team = self.team))]
  pub async fn send(self) -> octocrab::Result<EnterpriseTeam> {
    let route = format!(
      "/enterprises/{enterprise}/teams/{team_slug}",
      enterprise = self.enterprise,
      team_slug = self.team
    );
    self.client.get(route, Option::<&()>::None).await
  }
}

impl super::EnterpriseTeamsHandler<'_, '_> {
  pub fn get(&self, team: impl Into<String>) -> GetEnterpriseTeamRequest<'_> {
    GetEnterpriseTeamRequest::builder()
      .client(self.client)
      .enterprise(self.enterprise)
      .team(team)
      .build()
  }
}

impl super::EnterpriseTeamHandler<'_, '_, '_> {
  pub fn get(&self) -> GetEnterpriseTeamRequest<'_> {
    GetEnterpriseTeamRequest::builder()
      .client(self.client)
      .enterprise(self.enterprise)
      .team(self.team)
      .build()
  }
}
