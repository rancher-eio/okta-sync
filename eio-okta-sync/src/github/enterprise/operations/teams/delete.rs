use octocrab::Octocrab;

#[derive(bon::Builder)]
#[builder(on(String, into), state_mod = builder)]
pub struct DeleteEnterpriseTeamRequest<'client> {
  client: &'client Octocrab,
  enterprise: String,
  team: String,
}

impl DeleteEnterpriseTeamRequest<'_> {
  #[tracing::instrument(skip(self), fields(enterprise = self.enterprise, team = self.team))]
  pub async fn send(self) -> octocrab::Result<()> {
    let route = format!(
      "/enterprises/{enterprise}/teams/{team_slug}",
      enterprise = self.enterprise,
      team_slug = self.team
    );

    let response = self.client._delete(route, Option::<&()>::None).await?;

    if response.status().is_success() {
      Ok(())
    } else {
      octocrab::FromResponse::from_response(octocrab::map_github_error(response).await?).await
    }
  }
}

impl super::EnterpriseTeamsHandler<'_, '_> {
  pub fn delete(&self, team: impl Into<String>) -> DeleteEnterpriseTeamRequest<'_> {
    DeleteEnterpriseTeamRequest::builder()
      .client(self.client)
      .enterprise(self.enterprise)
      .team(team)
      .build()
  }
}

impl super::EnterpriseTeamHandler<'_, '_, '_> {
  pub fn delete(&self) -> DeleteEnterpriseTeamRequest<'_> {
    DeleteEnterpriseTeamRequest::builder()
      .client(self.client)
      .enterprise(self.enterprise)
      .team(self.team)
      .build()
  }
}
