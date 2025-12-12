use octocrab::Octocrab;

mod cost_centers;
mod teams;

#[derive(Debug, bon::Builder)]
pub struct EnterpriseHandler<'client, 'enterprise> {
  client: &'client Octocrab,
  enterprise: &'enterprise str,
}

impl super::Enterprise for Octocrab {
  fn enterprise<'enterprise>(&self, enterprise: &'enterprise str) -> EnterpriseHandler<'_, 'enterprise> {
    EnterpriseHandler::builder().client(self).enterprise(enterprise).build()
  }
}
