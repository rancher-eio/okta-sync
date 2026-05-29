use authentik_client::apis::Error;
use authentik_client::apis::core_api::{CoreGroupsListError, CoreGroupsRetrieveError};

mod list;
mod retrieve;

pub use list::CoreGroupsList;
pub use retrieve::CoreGroupsRetrieve;

#[derive(Debug, thiserror::Error, miette::Diagnostic)]
#[error(transparent)]
pub enum CoreGroupsApiError {
  #[diagnostic(code(authentik::api::core::groups::list))]
  List(#[from] Error<CoreGroupsListError>),
  #[diagnostic(code(authentik::api::core::groups::retrieve))]
  Retrieve(#[from] Error<CoreGroupsRetrieveError>),
}
