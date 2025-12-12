use octocrab::Octocrab;
use uuid::Uuid;

use crate::github::enterprise::models::AddedResourcesResponse;

#[derive(serde::Serialize, bon::Builder)]
#[builder(on(String, into), state_mod = builder)]
pub struct AddResourcesRequest<'client> {
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

impl AddResourcesRequest<'_> {
  pub async fn send(self) -> octocrab::Result<AddedResourcesResponse> {
    let route = format!(
      "/enterprises/{enterprise}/settings/billing/cost-centers/{cost_center_id}/resource",
      enterprise = self.enterprise,
      cost_center_id = self.cost_center_id
    );
    self.client.post(route, Some(&self)).await
  }
}

impl super::EnterpriseCostCenterHandler<'_, '_, '_> {
  pub fn add_resources(
    &self,
  ) -> AddResourcesRequestBuilder<'_, builder::SetCostCenterId<builder::SetEnterprise<builder::SetClient>>> {
    AddResourcesRequest::builder()
      .client(self.client)
      .enterprise(self.enterprise)
      .cost_center_id(*self.cost_center_id)
  }
}
