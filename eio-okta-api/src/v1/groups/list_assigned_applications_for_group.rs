use clap::Args;
use eio_okta_data::v2024_07_0::management::components::{
  parameters::{PathGroupId, QueryLimit},
  schemas::User,
};
use iri_string::template::{simple_context::SimpleContext, Context};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
  traits::{IntoRequest, Pagination},
  Endpoint, MapInto, QueryString,
};

impl Endpoint for ListAssignedApplicationsForGroup {
  const PATH: &'static str = "/api/v1/groups/{groupId}/apps";

  fn query(&self) -> Result<QueryString, crate::traits::endpoint::Error> {
    QueryString::simple(&self.query).map_into()
  }

  fn context(&self) -> impl Context {
    let mut context = SimpleContext::new();
    context.insert("groupId", self.path.group_id.as_str());
    context
  }
}

impl IntoRequest for ListAssignedApplicationsForGroup {
  type Body = ();
}

impl Pagination for ListAssignedApplicationsForGroup {
  type Item = User;
}

#[derive(Debug, Clone, Serialize, Deserialize, Args, Validate)]
#[remain::sorted]
pub struct ListAssignedApplicationsForGroup {
  #[command(flatten)]
  #[serde(flatten)]
  pub path: PathGroupId,
  #[command(flatten)]
  #[serde(flatten)]
  pub query: QueryLimit,
}

impl AsRef<()> for ListAssignedApplicationsForGroup {
  fn as_ref(&self) -> &() {
    &()
  }
}

impl Context for ListAssignedApplicationsForGroup {
  fn visit<V: iri_string::template::context::Visitor>(&self, visitor: V) -> V::Result {
    match visitor.var_name().as_str() {
      "groupId" => visitor.visit_string(&self.path.group_id),
      _ => visitor.visit_undefined(),
    }
  }
}
