use std::collections::HashSet;

use octocrab::Octocrab;

#[derive(serde::Serialize, bon::Builder)]
#[builder(on(String, into), state_mod = builder)]
pub struct RemoveOrganizationAssignmentsRequest<'client> {
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

impl RemoveOrganizationAssignmentsRequest<'_> {
  #[tracing::instrument(skip(self), fields(enterprise = self.enterprise, team = self.team, orgs = self.orgs.len()))]
  pub async fn send(self) -> octocrab::Result<()> {
    let route = format!(
      "/enterprises/{enterprise}/teams/{enterprise_team}/organizations/remove",
      enterprise = self.enterprise,
      enterprise_team = self.team
    );

    let response = self.client._post(route, Some(&self)).await?;

    if response.status().is_success() {
      Ok(())
    } else {
      octocrab::FromResponse::from_response(octocrab::map_github_error(response).await?).await
    }
  }
}

impl super::EnterpriseTeamOrganizationsHandler<'_, '_, '_> {
  pub fn bulk_remove(
    &self,
  ) -> RemoveOrganizationAssignmentsRequestBuilder<'_, builder::SetTeam<builder::SetEnterprise<builder::SetClient>>> {
    RemoveOrganizationAssignmentsRequest::builder()
      .client(self.client)
      .enterprise(self.enterprise)
      .team(self.team)
  }
}
