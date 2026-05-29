mod core;

pub(crate) use core::*;

#[derive(Debug, thiserror::Error, miette::Diagnostic)]
#[diagnostic(transparent)]
#[error(transparent)]
// each API can fail in different ways, but sometimes we just care that it was "an API error of some sort"
pub enum ApiError {
  Core(#[from] CoreApiError),
}
