use clap::Args;
use eio_okta_data::v2024_07_0::management::components::{
  parameters::{PathGroupId, QueryLimit},
  schemas::Group,
};
use iri_string::template::{simple_context::SimpleContext, Context};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
  query_string::QueryString,
  traits::{IntoRequest, Response},
  Endpoint, MapInto,
};

impl Endpoint for GetGroup {
  const PATH: &'static str = "/api/v1/groups/{groupId}";

  fn query(&self) -> Result<QueryString, crate::traits::endpoint::Error> {
    QueryString::simple(&self.query).map_into()
  }

  fn context(&self) -> impl Context {
    let mut context = SimpleContext::new();
    context.insert("groupId", self.path.group_id.as_str());
    context
  }
}

impl IntoRequest for GetGroup {
  type Body = ();
}

impl Response for GetGroup {
  type Body = Group;
}

#[derive(Debug, Clone, Serialize, Deserialize, Args, Validate)]
#[remain::sorted]
pub struct GetGroup {
  #[command(flatten)]
  #[serde(flatten)]
  pub path: PathGroupId,
  #[command(flatten)]
  #[serde(flatten)]
  pub query: QueryLimit,
}

impl AsRef<()> for GetGroup {
  fn as_ref(&self) -> &() {
    &()
  }
}

impl Context for GetGroup {
  fn visit<V: iri_string::template::context::Visitor>(&self, visitor: V) -> V::Result {
    match visitor.var_name().as_str() {
      "groupId" => visitor.visit_string(&self.path.group_id),
      _ => visitor.visit_undefined(),
    }
  }
}
