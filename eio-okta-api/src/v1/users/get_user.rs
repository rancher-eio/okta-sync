use clap::Args;
use eio_okta_data::v2024_07_0::management::components::{
  parameters::{PathUserId, QueryUserExpand},
  schemas::User,
};
use iri_string::template::{simple_context::SimpleContext, Context};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
  traits::{IntoRequest, Response},
  Endpoint, MapInto, QueryString,
};

impl Endpoint for GetUser {
  const PATH: &'static str = "/api/v1/users/{userId}";

  fn query(&self) -> Result<QueryString, crate::traits::endpoint::Error> {
    QueryString::simple(&self.query).map_into()
  }

  fn context(&self) -> impl Context {
    let mut context = SimpleContext::new();
    context.insert("userId", self.path.user_id.as_str());
    context
  }
}

impl IntoRequest for GetUser {
  type Body = ();
}

impl Response for GetUser {
  type Body = User;
}

#[derive(Debug, Clone, Serialize, Deserialize, Args, Validate)]
#[remain::sorted]
pub struct GetUser {
  #[command(flatten)]
  #[serde(flatten)]
  pub path: PathUserId,
  #[command(flatten)]
  #[serde(flatten)]
  pub query: QueryUserExpand,
}

impl AsRef<()> for GetUser {
  fn as_ref(&self) -> &() {
    &()
  }
}

impl Context for GetUser {
  fn visit<V: iri_string::template::context::Visitor>(&self, visitor: V) -> V::Result {
    match visitor.var_name().as_str() {
      "userId" => visitor.visit_string(&self.path.user_id),
      _ => visitor.visit_undefined(),
    }
  }
}
