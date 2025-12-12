use octocrab::Octocrab;
use uuid::Uuid;

use crate::github::enterprise::models::CostCenter;

#[derive(bon::Builder)]
#[builder(on(String, into), state_mod = builder)]
pub struct GetCostCenterRequest<'client> {
  client: &'client Octocrab,
  enterprise: String,
  cost_center_id: Uuid,
}

impl GetCostCenterRequest<'_> {
  pub async fn send(self) -> octocrab::Result<CostCenter> {
    let route = format!(
      "/enterprises/{enterprise}/settings/billing/cost-centers/{cost_center_id}",
      enterprise = self.enterprise,
      cost_center_id = self.cost_center_id
    );
    self.client.get(route, Option::<&()>::None).await
  }
}

impl super::EnterpriseCostCenterHandler<'_, '_, '_> {
  pub fn get(&self) -> GetCostCenterRequest<'_> {
    GetCostCenterRequest::builder()
      .client(self.client)
      .enterprise(self.enterprise)
      .cost_center_id(*self.cost_center_id)
      .build()
  }
}
