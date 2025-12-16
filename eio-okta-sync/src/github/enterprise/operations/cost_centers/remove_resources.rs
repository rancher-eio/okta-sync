use octocrab::Octocrab;
use uuid::Uuid;

use crate::github::enterprise::models::RemoveResourcesResponse;

#[derive(serde::Serialize, bon::Builder)]
#[builder(on(String, into), state_mod = builder)]
pub struct RemoveResourcesRequest<'client> {
  #[serde(skip)]
  client: &'client Octocrab,
  #[serde(skip)]
  enterprise: String,
  #[serde(skip)]
  cost_center_id: Uuid,
  #[builder(default)]
  #[serde(skip_serializing_if = "Vec::is_empty")]
  organizations: Vec<String>,
  #[builder(default)]
  #[serde(skip_serializing_if = "Vec::is_empty")]
  repositories: Vec<String>,
  #[builder(default)]
  #[serde(skip_serializing_if = "Vec::is_empty")]
  users: Vec<String>,
}

impl RemoveResourcesRequest<'_> {
  #[tracing::instrument(skip(self), fields(enterprise = self.enterprise, cost_center_id = self.cost_center_id.to_string(), organizations = self.organizations.len(), repositories = self.repositories.len(), users = self.users.len()))]
  pub async fn send(self) -> octocrab::Result<RemoveResourcesResponse> {
    let route = format!(
      "/enterprises/{enterprise}/settings/billing/cost-centers/{cost_center_id}/resource",
      enterprise = self.enterprise,
      cost_center_id = self.cost_center_id
    );
    self.client.delete(route, Some(&self)).await
  }
}

impl super::EnterpriseCostCentersHandler<'_, '_> {
  pub fn remove_resources(
    &self,
    cost_center_id: Uuid,
  ) -> RemoveResourcesRequestBuilder<'_, builder::SetCostCenterId<builder::SetEnterprise<builder::SetClient>>> {
    RemoveResourcesRequest::builder()
      .client(self.client)
      .enterprise(self.enterprise)
      .cost_center_id(cost_center_id)
  }
}

impl super::EnterpriseCostCenterHandler<'_, '_, '_> {
  pub fn remove_resources(
    &self,
  ) -> RemoveResourcesRequestBuilder<'_, builder::SetCostCenterId<builder::SetEnterprise<builder::SetClient>>> {
    RemoveResourcesRequest::builder()
      .client(self.client)
      .enterprise(self.enterprise)
      .cost_center_id(*self.cost_center_id)
  }
}
