use octocrab::Octocrab;
use uuid::Uuid;

use crate::github::enterprise::models::CostCenter;

#[derive(serde::Serialize, bon::Builder)]
#[builder(on(String, into), state_mod = builder)]
pub struct UpdateCostCenterRequest<'client> {
  #[serde(skip)]
  client: &'client Octocrab,
  #[serde(skip)]
  enterprise: String,
  #[serde(skip)]
  cost_center_id: Uuid,
  name: String,
}

impl UpdateCostCenterRequest<'_> {
  pub async fn send(self) -> octocrab::Result<CostCenter> {
    let route = format!(
      "/enterprises/{enterprise}/settings/billing/cost-centers/{cost_center_id}",
      enterprise = self.enterprise,
      cost_center_id = self.cost_center_id,
    );
    self.client.patch(route, Some(&self)).await
  }
}

impl super::EnterpriseCostCentersHandler<'_, '_> {
  pub fn update(&self) -> UpdateCostCenterRequestBuilder<'_, builder::SetEnterprise<builder::SetClient>> {
    UpdateCostCenterRequest::builder()
      .client(self.client)
      .enterprise(self.enterprise)
  }
}
