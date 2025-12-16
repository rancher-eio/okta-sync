use octocrab::Octocrab;

use crate::github::enterprise::models::CostCenter;

#[derive(serde::Serialize, bon::Builder)]
#[builder(on(String, into), state_mod = builder)]
pub struct CreateCostCenterRequest<'client> {
  #[serde(skip)]
  client: &'client Octocrab,
  #[serde(skip)]
  enterprise: String,
  name: String,
}

impl CreateCostCenterRequest<'_> {
  #[tracing::instrument(skip(self), fields(enterprise = self.enterprise, name = self.name))]
  pub async fn send(self) -> octocrab::Result<CostCenter> {
    let route = format!(
      "/enterprises/{enterprise}/settings/billing/cost-centers",
      enterprise = self.enterprise,
    );
    self.client.post(route, Some(&self)).await
  }
}

impl super::EnterpriseCostCentersHandler<'_, '_> {
  pub fn create(&self, name: impl Into<String>) -> CreateCostCenterRequest<'_> {
    CreateCostCenterRequest::builder()
      .client(self.client)
      .enterprise(self.enterprise)
      .name(name)
      .build()
  }
}
