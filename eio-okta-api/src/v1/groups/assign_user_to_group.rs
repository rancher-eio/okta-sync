use clap::Args;
use eio_okta_data::v2024_07_0::management::components::parameters::{PathGroupId, PathUserId};
use iri_string::template::Context;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
  Endpoint,
  traits::{IntoRequest, Response},
};

impl Endpoint for AssignUserToGroup {
  const PATH: &'static str = "/api/v1/groups/{groupId}/users/{userId}";

  fn context(&self) -> impl Context {
    self.clone()
  }
}

impl IntoRequest for AssignUserToGroup {
  const METHOD: http::Method = http::Method::PUT;
  type Body = ();
}

impl Response for AssignUserToGroup {
  type Body = ();
}

#[derive(Debug, Clone, Serialize, Deserialize, Args, Validate)]
#[remain::sorted]
pub struct AssignUserToGroup {
  #[command(flatten)]
  #[serde(flatten)]
  pub group: PathGroupId,
  #[command(flatten)]
  #[serde(flatten)]
  pub user: PathUserId,
}

impl AsRef<()> for AssignUserToGroup {
  fn as_ref(&self) -> &() {
    &()
  }
}

impl Context for AssignUserToGroup {
  fn visit<V: iri_string::template::context::Visitor>(&self, visitor: V) -> V::Result {
    match visitor.var_name().as_str() {
      "groupId" => visitor.visit_string(&self.group.group_id),
      "userId" => visitor.visit_string(&self.user.user_id),
      _ => visitor.visit_undefined(),
    }
  }
}
