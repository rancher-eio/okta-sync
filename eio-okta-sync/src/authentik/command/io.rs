mod input;
mod output;

use crate::authentik::macros::From;

pub(crate) use input::*;
pub(crate) use output::*;

#[derive(Debug, Clone, clap::Args)]
#[group(id = "io")]
pub(crate) struct IoOptions {
  #[command(flatten)]
  pub(crate) input: Input,
  #[command(flatten)]
  pub(crate) output: Output,
}

#[derive(Debug, thiserror::Error, miette::Diagnostic)]
#[diagnostic(transparent)]
#[error(transparent)]
#[remain::sorted]
pub enum IoError {
  Input(#[from] InputError),
  Output(#[from] OutputError),
}

const _: () = {
  use super::super::Error;
  From!(InputFormatError => InputError => IoError => Error);
  From!(InputSourceError => InputError => IoError => Error);
  From!(OutputFormatError => OutputError => IoError => Error);
  From!(OutputTargetError => OutputError => IoError => Error);
  From!(InputError => IoError => Error);
  From!(OutputError => IoError => Error);
};
