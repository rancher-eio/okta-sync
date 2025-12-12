use octocrab::Octocrab;
use uuid::Uuid;

mod add_resources;
mod create;
mod delete;
mod get;
mod list;
mod remove_resources;
mod update;

#[derive(bon::Builder)]
pub struct EnterpriseCostCentersHandler<'client, 'enterprise> {
  client: &'client Octocrab,
  enterprise: &'enterprise str,
}

impl super::EnterpriseHandler<'_, '_> {
  pub fn cost_centers(&self) -> EnterpriseCostCentersHandler<'_, '_> {
    EnterpriseCostCentersHandler::builder()
      .client(self.client)
      .enterprise(self.enterprise)
      .build()
  }
}

#[derive(bon::Builder)]
pub struct EnterpriseCostCenterHandler<'client, 'enterprise, 'cost_center> {
  client: &'client Octocrab,
  enterprise: &'enterprise str,
  cost_center_id: &'cost_center Uuid,
}

impl super::EnterpriseHandler<'_, '_> {
  pub fn cost_center<'cost_center>(
    &self,
    cost_center_id: &'cost_center Uuid,
  ) -> EnterpriseCostCenterHandler<'_, '_, 'cost_center> {
    EnterpriseCostCenterHandler::builder()
      .client(self.client)
      .enterprise(self.enterprise)
      .cost_center_id(cost_center_id)
      .build()
  }
}
