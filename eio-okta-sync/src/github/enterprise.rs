pub mod models;
mod operations;

use operations::EnterpriseHandler;

pub trait Enterprise {
  fn enterprise<'enterprise>(&self, enterprise: &'enterprise str) -> EnterpriseHandler<'_, 'enterprise>;
}
