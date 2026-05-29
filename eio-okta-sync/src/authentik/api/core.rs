mod groups;
mod tokens;
mod users;

pub use groups::*;
pub use tokens::*;
pub use users::*;

#[derive(Debug, thiserror::Error, miette::Diagnostic)]
#[diagnostic(transparent)]
#[error(transparent)]
// each API has its own ways of failing, but here we just care that "some core API failed".
pub enum CoreApiError {
  Groups(#[from] groups::CoreGroupsApiError),
  Tokens(#[from] tokens::CoreTokensApiError),
  Users(#[from] users::CoreUsersApiError),
}
