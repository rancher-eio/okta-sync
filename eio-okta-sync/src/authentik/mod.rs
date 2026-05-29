use std::borrow::Borrow;

use authentik_client::apis::configuration::Configuration;

mod api;
mod command;
mod date_time;
mod macros;
mod options;
mod prelude;
pub(crate) mod traits;

pub(crate) use command::Command;
pub(crate) use date_time::HumanFriendlyDateTime;

#[derive(Debug, thiserror::Error, miette::Diagnostic)]
#[error(transparent)]
/// there are many ways things can fail, and they should all be able to convert to this.
pub enum Error {
  #[diagnostic(transparent)]
  Api(#[from] api::ApiError),
  #[diagnostic(code(clap))]
  Clap(#[from] clap::Error),
  #[diagnostic(transparent)]
  IO(#[from] command::io::IoError),
  #[diagnostic(code(json))]
  Json(#[from] serde_json::Error),
}

#[derive(Debug, Clone, bon::Builder)]
pub(crate) struct Client {
  configuration: Configuration,
}

impl Borrow<Configuration> for Client {
  fn borrow(&self) -> &Configuration {
    &self.configuration
  }
}

impl AsRef<Configuration> for Client {
  fn as_ref(&self) -> &Configuration {
    &self.configuration
  }
}
