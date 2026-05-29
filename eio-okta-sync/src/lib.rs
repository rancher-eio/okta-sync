#![allow(clippy::result_large_err)]

pub(crate) mod args;
pub(crate) mod authentik;
pub mod command;
pub mod crossplane;
pub mod github;
pub(crate) mod graphql;
pub mod kubernetes;
pub(crate) mod macros;
pub mod okta;

#[derive(Debug, thiserror::Error, miette::Diagnostic)]
#[error(transparent)]
#[allow(clippy::large_enum_variant)]
#[remain::sorted]
#[non_exhaustive]
pub enum Error {
  #[diagnostic(code(argument::validation::betrayal))]
  #[error("{betrayal}. CLI argument validation should have prevented this. (╯°□°)╯︵ ┻━┻")]
  ArgumentValidationFailedToPrevent { betrayal: &'static str },
  #[diagnostic(transparent)]
  Authentik(#[from] authentik::Error),
  #[diagnostic(transparent)]
  Client(#[from] eio_okta_client::Error),
  #[diagnostic(code(response::data::subpar))]
  #[error("response data was displeasing because: {reason}. Sorry you had to witness this. 😒")]
  DispleasingResponseData { reason: &'static str },
  #[diagnostic(code(::fancy_regex))]
  FancyRegex(#[from] fancy_regex::Error),
  #[diagnostic(code(graphql))]
  #[error("graphql response includes error messages: {0:#?}")]
  GraphQL(Vec<String>),
  #[diagnostic(code(::inquire))]
  Inquire(#[from] inquire::error::InquireError),
  #[diagnostic(code(::std::io))]
  IO(#[from] std::io::Error),
  #[diagnostic(code(::iri_string::template))]
  IriStringTemplate(#[from] iri_string::template::Error),
  #[diagnostic(code(::iri_string::validate))]
  IriStringValidate(#[from] iri_string::validate::Error),
  #[diagnostic(code(::serde_json))]
  Json(#[from] serde_json::Error),
  #[diagnostic(code(credentials::github::token::missing))]
  #[error("missing github access token")]
  MissingGithubAccessToken,
  #[diagnostic(code(::octocrab))]
  Octocrab(#[from] octocrab::Error),
  #[diagnostic(code(::serde_yml))]
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
