use bounded_integer::BoundedU8;
use octocrab::Octocrab;

use crate::github::enterprise::models::EnterpriseTeam;

#[derive(serde::Serialize, bon::Builder)]
#[builder(on(String, into), state_mod = builder)]
pub struct ListEnterpriseTeamsRequest<'client> {
  #[serde(skip)]
  client: &'client Octocrab,
  #[serde(skip)]
  enterprise: String,
  #[builder(default = BoundedU8::const_new::<30>())]
  per_page: BoundedU8<0, 100>,
  #[builder(default = 1)]
  page: usize,
}

impl ListEnterpriseTeamsRequest<'_> {
  pub async fn send(self) -> octocrab::Result<octocrab::Page<EnterpriseTeam>> {
    let route = format!("/enterprises/{enterprise}/teams", enterprise = self.enterprise);
    self.client.get(route, Some(&self)).await
  }
}

impl super::EnterpriseTeamsHandler<'_, '_> {
  pub fn list(&self) -> ListEnterpriseTeamsRequestBuilder<'_, builder::SetEnterprise<builder::SetClient>> {
    ListEnterpriseTeamsRequest::builder()
      .client(self.client)
      .enterprise(self.enterprise)
  }
}
