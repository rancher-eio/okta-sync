use serde::{de::DeserializeOwned, Serialize};

pub trait Response {
  type Body: Clone + Serialize + DeserializeOwned + std::fmt::Debug;
}
