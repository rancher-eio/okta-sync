pub mod authorization;
pub mod qol;
pub mod query_string;
pub mod traits;
pub mod v1;
pub use eio_okta_data as data;

pub use query_string::QueryString;
pub use traits::Endpoint;
pub use traits::Service;

pub trait MapInto<T, E> {
  fn map_into(self) -> Result<T, E>;
}

impl<OkFrom, ErrFrom, OkInto, ErrInto> MapInto<OkInto, ErrInto> for Result<OkFrom, ErrFrom>
where
  OkFrom: Into<OkInto>,
  ErrFrom: Into<ErrInto>,
{
  fn map_into(self) -> Result<OkInto, ErrInto> {
    self.map(Into::into).map_err(Into::into)
  }
}
