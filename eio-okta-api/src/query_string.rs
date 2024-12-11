use std::ops::Deref;

use iri_string::types::{UriQueryStr, UriQueryString};
use serde::Serialize;

#[derive(Debug, thiserror::Error)]
#[remain::sorted]
pub enum Error {
  #[error("complex: {0}")]
  Complex(#[from] serde_qs::Error),
  #[error("invalid: {0}")]
  Invalid(#[from] iri_string::validate::Error),
  #[error("simple: {0}")]
  Simple(#[from] serde_urlencoded::ser::Error),
  #[error(transparent)]
  Validation(#[from] validator::ValidationErrors),
}

pub struct QueryString(UriQueryString);

impl Deref for QueryString {
  type Target = UriQueryStr;
  fn deref(&self) -> &Self::Target {
    self.0.deref()
  }
}

impl QueryString {
  pub fn simple<T>(input: &T) -> Result<Self, Error>
  where
    T: Serialize,
  {
    let s = serde_urlencoded::to_string(input)?;
    Ok(Self(UriQueryStr::new(&s)?.into()))
  }

  pub fn nested<T>(input: &T) -> Result<Self, Error>
  where
    T: Serialize,
  {
    let s = serde_qs::to_string(input)?;
    Ok(Self(UriQueryStr::new(&s)?.into()))
  }
}
