use std::{collections::BTreeMap, fs::File, path::PathBuf};

use comparable::Comparable;
use eio_okta_data::v2024_07_0::management::components::schemas::{Group, User};

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

  let user1 = snapshot.users.iter().find(|user| user.profile.email.eq("user-1@company.com")).unwrap();
  let user2 = snapshot.users.iter().find(|user| user.profile.email.eq("user-2@company.com")).unwrap();

  let diff = user1.comparison(&user2);

  println!("DIFF: {:?}", diff);
}
