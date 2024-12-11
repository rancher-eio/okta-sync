mod authorization_ssws;
pub mod endpoint;
mod into_request;
mod pagination;
mod response;
pub mod service;

pub use authorization_ssws::AuthorizationSSWS;
pub use endpoint::Endpoint;
pub use into_request::IntoRequest;
pub use pagination::Pagination;
pub use response::Response;
pub use service::Service;

impl<T> Response for T
where
  T: Pagination,
{
  type Body = Vec<<Self as Pagination>::Item>;
}
