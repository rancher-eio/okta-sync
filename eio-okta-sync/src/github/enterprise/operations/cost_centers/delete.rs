use octocrab::Octocrab;
use uuid::Uuid;

use crate::github::enterprise::models::DeleteCostCenterResponse;

#[derive(bon::Builder)]
#[builder(on(String, into), state_mod = builder)]
pub struct DeleteCostCenterRequest<'client> {
  client: &'client Octocrab,
  enterprise: String,
  cost_center_id: Uuid,
}

impl DeleteCostCenterRequest<'_> {
  #[tracing::instrument(skip(self), fields(enterprise = self.enterprise, cost_center_id = self.cost_center_id.to_string()))]
  pub async fn send(self) -> octocrab::Result<DeleteCostCenterResponse> {
    let route = format!(
      "/enterprises/{enterprise}/settings/billing/cost-centers/{cost_center_id}",
      enterprise = self.enterprise,
      cost_center_id = self.cost_center_id,
    );
    self.client.delete(route, Option::<&()>::None).await
  }
}

impl super::EnterpriseCostCentersHandler<'_, '_> {
  pub fn delete(&self, cost_center_id: Uuid) -> DeleteCostCenterRequest<'_> {
    DeleteCostCenterRequest::builder()
      .client(self.client)
      .enterprise(self.enterprise)
      .cost_center_id(cost_center_id)
      .build()
  }
}

impl super::EnterpriseCostCenterHandler<'_, '_, '_> {
  pub fn delete(&self) -> DeleteCostCenterRequest<'_> {
    DeleteCostCenterRequest::builder()
      .client(self.client)
      .enterprise(self.enterprise)
      .cost_center_id(*self.cost_center_id)
      .build()
  }
}
