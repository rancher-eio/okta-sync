#![allow(clippy::result_large_err)]

pub(crate) mod client;
pub mod command;
pub(crate) mod options;

pub use client::Client;
pub use command::Command;
pub use options::*;

#[derive(Debug, thiserror::Error, miette::Diagnostic)]
#[allow(clippy::large_enum_variant)]
#[remain::sorted]
pub enum Error {
  #[error(transparent)]
  Endpoint(#[from] eio_okta_api::traits::endpoint::Error),
  #[error(transparent)]
  Http(#[from] http::Error),
  #[error(transparent)]
  IO(#[from] std::io::Error),
  #[error(transparent)]
  Json(#[from] serde_json::Error),
  #[error(transparent)]
  ParseLinkHeader(#[from] parse_link_header::Error),
  #[error(transparent)]
  Service(#[from] eio_okta_api::traits::service::Error),
  #[error(transparent)]
  Ureq(#[from] ureq::Error),
}

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
