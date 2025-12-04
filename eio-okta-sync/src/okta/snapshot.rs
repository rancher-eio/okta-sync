use std::collections::BTreeMap;
use std::path::Path;

use eio_okta_data::v2024_07_0::management::components::schemas::{Group, User};

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct Snapshot {
  pub groups: Vec<Group>,
  pub group_users: BTreeMap<String, Vec<User>>,
  pub users: Vec<User>,
}

impl Snapshot {
  pub(crate) fn read_from_file(path: impl AsRef<Path>) -> Result<Self, crate::Error> {
    let yaml = fs_err::read_to_string(path)?;
    let this = serde_yml::from_str(&yaml)?;
    Ok(this)
  }
}
