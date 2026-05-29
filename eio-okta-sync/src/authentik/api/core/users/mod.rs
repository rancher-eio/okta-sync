use authentik_client::apis::Error;
use authentik_client::apis::core_api::{CoreUsersListError, CoreUsersMeRetrieveError, CoreUsersRetrieveError};

mod list;
mod me;
mod retrieve;

pub use list::CoreUsersList;
pub use me::CoreUsersMeRetrieve;
pub use retrieve::CoreUsersRetrieve;

#[derive(Debug, thiserror::Error, miette::Diagnostic)]
#[error(transparent)]
pub enum CoreUsersApiError {
  #[diagnostic(code(authentik::api::core::users::list))]
  List(#[from] Error<CoreUsersListError>),
  #[diagnostic(code(authentik::api::core::users::me::retrieve))]
  MeRetrieve(#[from] Error<CoreUsersMeRetrieveError>),
  #[diagnostic(code(authentik::api::core::users::retrieve))]
  Retrieve(#[from] Error<CoreUsersRetrieveError>),
}
