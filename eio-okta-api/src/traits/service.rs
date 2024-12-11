use std::ops::Deref;

use http::uri::{Authority, Scheme};
use iri_string::types::{UriAbsoluteStr, UriAbsoluteString};

#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error("http: {0}")]
  Http(#[from] http::Error),
  #[error("endpoint: {0}")]
  Endpoint(#[from] super::endpoint::Error),
  #[error("validate: {0}")]
  Validate(#[from] iri_string::validate::Error),
}

pub trait Service: Sized {
  fn scheme(&self) -> &Scheme {
    &Scheme::HTTPS
  }

  fn authority(&self) -> &Authority;

  fn uri(&self) -> Result<ServiceURI, Error> {
    ServiceURI::new(self)
  }
}

#[derive(Debug, Clone)]
pub struct ServiceURI(UriAbsoluteString);

impl Deref for ServiceURI {
  type Target = UriAbsoluteStr;
  fn deref(&self) -> &Self::Target {
    self.0.deref()
  }
}

impl ServiceURI {
  fn new<T>(service: &T) -> Result<Self, Error>
  where
    T: Service,
  {
    let mut uri = iri_string::build::Builder::new();

    uri.scheme(service.scheme().as_str());
    uri.host(service.authority().host());
    if let Some(port) = service.authority().port_u16() {
      uri.port(port)
    }
    uri.path("");
    uri.unset_fragment();
    uri.unset_query();
    uri.unset_userinfo();
    uri.normalize();

    Ok(Self(uri.build::<UriAbsoluteStr>()?.into()))
  }
}
