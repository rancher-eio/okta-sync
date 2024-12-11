use http::Method;
use serde::{de::DeserializeOwned, Serialize};

pub trait IntoRequest: AsRef<Self::Body> {
  const METHOD: Method = Method::GET;
  type Body: Clone + Serialize + DeserializeOwned + std::fmt::Debug;
}
