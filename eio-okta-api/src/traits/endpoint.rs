use std::ops::Deref;

use crate::MapInto;
use headers::ContentType;
use http::Version;
use iri_string::{
  build::Builder,
  spec::UriSpec,
  template::{simple_context::SimpleContext, Context, UriTemplateStr},
  types::{UriRelativeStr, UriRelativeString},
};

use crate::query_string::QueryString;

use super::{IntoRequest, Response};

#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error("query string: {0}")]
  QueryString(#[from] crate::query_string::Error),
  #[error("template: {0}")]
  Template(#[from] iri_string::template::Error),
  #[error("validate: {0}")]
  Validate(#[from] iri_string::validate::Error),
}

pub trait Endpoint
where
  Self: Sized + IntoRequest + Response + AsRef<<Self as IntoRequest>::Body>,
{
  const PATH: &'static str;
  const VERSION: Version = Version::HTTP_11;

  fn accept(&self) -> ContentType {
    ContentType::json()
  }

  fn content_type(&self) -> ContentType {
    ContentType::json()
  }

  fn context(&self) -> impl Context {
    SimpleContext::new()
  }

  fn query(&self) -> Result<QueryString, Error> {
    QueryString::simple(&()).map_into()
  }

  fn path(&self) -> Result<EndpointPath, Error> {
    EndpointPath::new(self)
  }

  fn uri(&self) -> Result<UriRelativeString, Error> {
    let path = self.path()?;
    let query = self.query()?;
    let mut uri = Builder::new();
    uri.path(path.path_str());
    uri.query(query.as_str());
    Ok(uri.build::<UriRelativeStr>()?.into())
  }
}

#[derive(Debug)]
pub struct EndpointPath(UriRelativeString);

impl Deref for EndpointPath {
  type Target = UriRelativeStr;
  fn deref(&self) -> &Self::Target {
    self.0.deref()
  }
}

impl EndpointPath {
  fn new<T>(endpoint: &T) -> Result<Self, Error>
  where
    T: Endpoint,
  {
    let template = UriTemplateStr::new(T::PATH)?;
    let context = endpoint.context();
    let expanded = template.expand::<UriSpec, _>(&context)?;
    let rendered = expanded.to_string();

    let mut uri = Builder::new();
    uri.path(&rendered);
    uri.unset_authority();
    uri.unset_fragment();
    uri.unset_port();
    uri.unset_query();
    uri.unset_scheme();
    uri.unset_userinfo();
    uri.normalize();

    Ok(Self(uri.build::<UriRelativeStr>()?.into()))
  }
}
