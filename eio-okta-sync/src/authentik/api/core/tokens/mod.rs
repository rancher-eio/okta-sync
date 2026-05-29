use authentik_client::apis::Error;
use authentik_client::apis::core_api::{CoreTokensListError, CoreTokensRetrieveError};

mod list;
mod retrieve;

pub use list::CoreTokensList;
pub use retrieve::CoreTokensRetrieve;

#[derive(Debug, thiserror::Error, miette::Diagnostic)]
#[error(transparent)]
pub enum CoreTokensApiError {
  #[diagnostic(code(authentik::api::core::tokens::list))]
  List(#[from] Error<CoreTokensListError>),
  #[diagnostic(code(authentik::api::core::tokens::retrieve))]
  Retrieve(#[from] Error<CoreTokensRetrieveError>),
}
