use serde::{de::DeserializeOwned, Serialize};

pub trait Pagination {
  const HEADER: http::header::HeaderName = http::header::LINK;
  const LIMIT: usize = 200;
  const QUERY: &'static str = "after";
  const REL: &'static str = "next";
  type Item: Clone + Serialize + DeserializeOwned + std::fmt::Debug;
}
