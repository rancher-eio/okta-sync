use octocrab::Octocrab;

#[derive(bon::Builder)]
#[builder(on(String, into), state_mod = builder)]
pub struct RemoveOrganizationAssignmentRequest<'client> {
  client: &'client Octocrab,
  enterprise: String,
  team: String,
  org: String,
}

impl RemoveOrganizationAssignmentRequest<'_> {
  #[tracing::instrument(skip(self), fields(enterprise = self.enterprise, team = self.team, org = self.org))]
  pub async fn send(self) -> octocrab::Result<()> {
    let route = format!(
      "/enterprises/{enterprise}/teams/{enterprise_team}/organizations/{org}",
      enterprise = self.enterprise,
      enterprise_team = self.team,
      org = self.org,
    );

    let response = self.client._delete(route, Option::<&()>::None).await?;

    if response.status().is_success() {
      Ok(())
    } else {
      octocrab::FromResponse::from_response(octocrab::map_github_error(response).await?).await
    }
  }
}

impl super::EnterpriseTeamOrganizationsHandler<'_, '_, '_> {
  pub fn remove(&self, org: impl Into<String>) -> RemoveOrganizationAssignmentRequest<'_> {
    RemoveOrganizationAssignmentRequest::builder()
      .client(self.client)
      .enterprise(self.enterprise)
      .team(self.team)
      .org(org)
      .build()
  }
}

impl super::EnterpriseTeamOrganizationHandler<'_, '_, '_, '_> {
  pub fn remove(&self) -> RemoveOrganizationAssignmentRequest<'_> {
    RemoveOrganizationAssignmentRequest::builder()
      .client(self.client)
      .enterprise(self.enterprise)
      .team(self.team)
      .org(self.org)
      .build()
  }
}
