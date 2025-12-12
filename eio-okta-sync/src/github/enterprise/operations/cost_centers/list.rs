use octocrab::Octocrab;

use crate::github::enterprise::models::{CostCenterState, ListCostCentersResponse};

#[derive(serde::Serialize, bon::Builder)]
#[builder(on(String, into), state_mod = builder)]
pub struct ListCostCentersRequest<'client> {
  #[serde(skip)]
  client: &'client Octocrab,
  #[serde(skip)]
  enterprise: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  state: Option<CostCenterState>,
}

impl ListCostCentersRequest<'_> {
  pub async fn send(self) -> octocrab::Result<ListCostCentersResponse> {
    let route = format!(
      "/enterprises/{enterprise}/settings/billing/cost-centers",
      enterprise = self.enterprise
    );
    self.client.get(route, Some(&self)).await
  }
}

impl super::EnterpriseCostCentersHandler<'_, '_> {
  pub fn list(&self) -> ListCostCentersRequestBuilder<'_, builder::SetEnterprise<builder::SetClient>> {
    ListCostCentersRequest::builder()
      .client(self.client)
      .enterprise(self.enterprise)
  }
}
