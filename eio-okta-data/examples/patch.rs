use std::{collections::BTreeMap, fs::File, path::PathBuf};

use comparable::Comparable;
use eio_okta_data::v2024_07_0::management::components::schemas::{user_profile::UserProfilePatch, Group, User};

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct Snapshot {
  pub groups: Vec<Group>,
  pub group_users: BTreeMap<String, Vec<User>>,
  pub users: Vec<User>,
}

fn main() {
  let path = PathBuf::from("../../snapshot.yaml");
  let file = File::open(path).unwrap();
  let snapshot: Snapshot = serde_yml::from_reader(file).unwrap();

  let user = snapshot.users.iter().find(|user| user.profile.email.eq("user@company.com")).unwrap().to_owned();

  let patch = UserProfilePatch {
    employee_number: Some(Some("31337".into())),
    ..Default::default()
  };

  let patched = user.profile.clone() << patch.clone();

  let diff = user.profile.comparison(&patched);

  println!("DIFF: {:#?}", &diff);

  let json = serde_yml::to_string(&patched).unwrap();

  println!("{json}");
}
