pub(crate) mod graph;
mod snapshot;
mod user_profile;

use eio_okta_data::current::management::components::schemas::{User, UserProfile};

pub use snapshot::Snapshot;
pub use user_profile::UserProfileExtensions;

pub(crate) trait DisplayName {
  fn display_name(&self) -> String;
}

impl DisplayName for User {
  fn display_name(&self) -> String {
    match self.profile.as_ref() {
      UserProfile {
        first_name: Some(first_name),
        last_name: Some(last_name),
        ..
      } => format!("{first_name} {last_name}"),
      UserProfile {
        first_name: Some(first_name),
        last_name: None,
        ..
      } => format!("{first_name} MissingLastName"),
      UserProfile {
        first_name: None,
        last_name: Some(last_name),
        ..
      } => format!("MissingFirstName {last_name}"),
      UserProfile {
        first_name: None,
        last_name: None,
        ..
      } => String::from("MissingFirstName MissingLastName"),
    }
  }
}
