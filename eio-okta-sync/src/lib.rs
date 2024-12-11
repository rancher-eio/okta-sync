#![allow(clippy::result_large_err)]

pub mod command;
pub mod crossplane;
pub mod github;
pub mod kubernetes;
pub mod okta;

#[derive(Debug, thiserror::Error, miette::Diagnostic)]
#[error(transparent)]
#[allow(clippy::large_enum_variant)]
#[remain::sorted]
pub enum Error {
  Client(#[from] eio_okta_client::Error),
  IO(#[from] std::io::Error),
  IriStringTemplate(#[from] iri_string::template::Error),
  IriStringValidate(#[from] iri_string::validate::Error),
  Json(#[from] serde_json::Error),
  #[error("missing github access token")]
  MissingGithubAccessToken,
  Octocrab(#[from] octocrab::Error),
  Yaml(#[from] serde_yml::Error),
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
