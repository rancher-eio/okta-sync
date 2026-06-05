use authentik_client::apis::Error;
use authentik_client::apis::core_api::{
  CoreGroupsAddUserCreateError, CoreGroupsCreateError, CoreGroupsDestroyError, CoreGroupsListError,
  CoreGroupsPartialUpdateError, CoreGroupsRemoveUserCreateError, CoreGroupsRetrieveError, CoreGroupsUpdateError,
  CoreGroupsUsedByListError,
};

mod add_user_create;
mod create;
mod destroy;
mod list;
mod partial_update;
mod remove_user_create;
mod retrieve;
mod update;
mod used_by_list;

pub use add_user_create::CoreGroupsAddUserCreate;
pub use create::CoreGroupsCreate;
pub use destroy::CoreGroupsDestroy;
pub use list::CoreGroupsList;
pub use partial_update::CoreGroupsPartialUpdate;
pub use remove_user_create::CoreGroupsRemoveUserCreate;
pub use retrieve::CoreGroupsRetrieve;
pub use update::CoreGroupsUpdate;
pub use used_by_list::CoreGroupsUsedByList;

#[derive(Debug, thiserror::Error, miette::Diagnostic)]
#[error(transparent)]
pub enum CoreGroupsApiError {
  #[diagnostic(code(authentik::api::core::groups::add_user_create))]
  AddUserCreate(#[from] Error<CoreGroupsAddUserCreateError>),
  #[diagnostic(code(authentik::api::core::groups::create))]
  Create(#[from] Error<CoreGroupsCreateError>),
  #[diagnostic(code(authentik::api::core::groups::destroy))]
  Destroy(#[from] Error<CoreGroupsDestroyError>),
  #[diagnostic(code(authentik::api::core::groups::list))]
  List(#[from] Error<CoreGroupsListError>),
  #[diagnostic(code(authentik::api::core::groups::partial_update))]
  PartialUpdate(#[from] Error<CoreGroupsPartialUpdateError>),
  #[diagnostic(code(authentik::api::core::groups::remove_user_create))]
  RemoveUserCreate(#[from] Error<CoreGroupsRemoveUserCreateError>),
  #[diagnostic(code(authentik::api::core::groups::retrieve))]
  Retrieve(#[from] Error<CoreGroupsRetrieveError>),
  #[diagnostic(code(authentik::api::core::groups::update))]
  Update(#[from] Error<CoreGroupsUpdateError>),
  #[diagnostic(code(authentik::api::core::groups::used_by_list))]
  UsedByList(#[from] Error<CoreGroupsUsedByListError>),
}
