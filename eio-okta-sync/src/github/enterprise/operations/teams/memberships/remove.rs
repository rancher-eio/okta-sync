use octocrab::Octocrab;

#[derive(bon::Builder)]
#[builder(on(String, into), state_mod = builder)]
pub struct RemoveEnterpriseTeamMembershipRequest<'client> {
  client: &'client Octocrab,
  enterprise: String,
  team: String,
  username: String,
}

impl RemoveEnterpriseTeamMembershipRequest<'_> {
  pub async fn send(self) -> octocrab::Result<()> {
    let route = format!(
      "/enterprises/{enterprise}/teams/{enterprise_team}/memberships/{username}",
      enterprise = self.enterprise,
      enterprise_team = self.team,
      username = self.username,
    );

    let response = self.client._delete(route, Option::<&()>::None).await?;

    if response.status().is_success() {
      Ok(())
    } else {
      octocrab::FromResponse::from_response(octocrab::map_github_error(response).await?).await
    }
  }
}

impl super::EnterpriseTeamMembershipHandler<'_, '_, '_, '_> {
  pub fn delete(&self) -> RemoveEnterpriseTeamMembershipRequest<'_> {
    RemoveEnterpriseTeamMembershipRequest::builder()
      .client(self.client)
      .enterprise(self.enterprise)
      .team(self.team)
      .username(self.username)
      .build()
  }
}
