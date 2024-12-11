use std::collections::BTreeMap;

use eio_okta_data::v2024_07_0::management::components::schemas::{Group, User};

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct Snapshot {
  pub groups: Vec<Group>,
  pub group_users: BTreeMap<String, Vec<User>>,
  pub users: Vec<User>,
}
