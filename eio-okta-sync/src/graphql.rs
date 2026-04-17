#![allow(unused)]

use graphql_client::GraphQLQuery;
use serde_json::{Map, Number, Value};

pub(crate) mod macros;
pub(crate) use macros::*;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct MultiPageResponse<T> {
  pub pages: Vec<SinglePageResponse<T>>,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct SinglePageResponse<T> {
  pub data: T,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub errors: Option<Vec<Error>>,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct Error {
  pub path: Vec<ErrorPathSegment>,
  pub extensions: Option<Map<String, Value>>,
  pub locations: Vec<ErrorLocation>,
  pub message: String,
}

#[derive(Debug, Clone, Copy, serde::Deserialize, serde::Serialize)]
pub(crate) struct ErrorLocation {
  pub line: usize,
  pub column: usize,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(untagged)]
pub enum ErrorPathSegment {
  String(String),
  Number(Number),
}

pub(crate) trait PageInfo {
  type PageInfo;

  fn page_info(&self) -> Option<&Self::PageInfo>;
}

pub(crate) trait PageSize {
  const DEFAULT: i64 = Self::MAXIMUM;
  const MINIMUM: i64 = 0;
  const MAXIMUM: i64 = 100;

  fn enforce(page_size: &mut Option<i64>) {
    let value = page_size.unwrap_or(Self::DEFAULT).clamp(Self::MINIMUM, Self::MAXIMUM);
    page_size.replace(value);
  }
}

pub(crate) trait EndCursor {
  fn end_cursor(&self) -> Option<&str>;
}
pub(crate) trait HasNextPage {
  fn has_next_page(&self) -> bool;
}

pub(crate) trait ForwardPageInfo: EndCursor + HasNextPage {}

pub(crate) trait After {
  fn after(&mut self) -> &mut Option<String>;
}

pub(crate) trait First {
  fn first(&mut self) -> &mut Option<i64>;
}

pub(crate) trait ForwardPageVariables: After + First {}

pub(crate) trait PageForward: GraphQLQuery {
  fn page_forward(variables: &mut Self::Variables, response_data: &Self::ResponseData) -> bool;
}

impl<T> ForwardPageVariables for T where T: After + First {}

impl<T> ForwardPageInfo for T where T: EndCursor + HasNextPage {}

impl<T> PageForward for T
where
  T: GraphQLQuery,
  T::ResponseData: PageInfo,
  <T::ResponseData as PageInfo>::PageInfo: ForwardPageInfo,
  T::Variables: ForwardPageVariables,
{
  fn page_forward(variables: &mut Self::Variables, response_data: &Self::ResponseData) -> bool {
    if let Some(page) = response_data.page_info()
      && let Some(end_cursor) = page.end_cursor()
      && page.has_next_page()
    {
      variables.after().replace(end_cursor.into());
      true
    } else {
      false
    }
  }
}

pub(crate) trait OnePage {
  async fn graphql_one_page<T: GraphQLQuery>(
    &self,
    variables: T::Variables,
  ) -> Result<SinglePageResponse<T::ResponseData>, crate::Error>;
}

pub(crate) trait AllPagesForward: OnePage {
  async fn graphql_all_pages_forward<T: GraphQLQuery + PageForward + PageSize>(
    &self,
    mut variables: T::Variables,
  ) -> Result<MultiPageResponse<T::ResponseData>, crate::Error>
  where
    T::Variables: ForwardPageVariables + Clone,
  {
    T::enforce(variables.first());

    let mut pages = Vec::new();
    let mut next = true;

    while next {
      let response = self.graphql_one_page::<T>(variables.clone()).await?;
      next = T::page_forward(&mut variables, &response.data);
      pages.push(response);
    }

    Ok(MultiPageResponse { pages })
  }
}

impl<T> AllPagesForward for T where T: OnePage {}

impl<T> IntoIterator for SinglePageResponse<T>
where
  T: IntoIterator,
{
  type IntoIter = T::IntoIter;
  type Item = T::Item;

  fn into_iter(self) -> Self::IntoIter {
    self.data.into_iter()
  }
}

impl<'response, T> IntoIterator for &'response SinglePageResponse<T>
where
  &'response T: IntoIterator,
{
  type IntoIter = <&'response T as IntoIterator>::IntoIter;
  type Item = <&'response T as IntoIterator>::Item;

  fn into_iter(self) -> Self::IntoIter {
    (&self.data).into_iter()
  }
}

impl<T> IntoIterator for MultiPageResponse<T>
where
  SinglePageResponse<T>: IntoIterator,
{
  type IntoIter = std::vec::IntoIter<Self::Item>;
  type Item = <SinglePageResponse<T> as IntoIterator>::Item;

  fn into_iter(self) -> Self::IntoIter {
    self
      .pages
      .into_iter()
      .flat_map(|page| page.into_iter().collect::<Vec<_>>())
      .collect::<Vec<_>>()
      .into_iter()
  }
}

impl<'response, T> IntoIterator for &'response MultiPageResponse<T>
where
  &'response SinglePageResponse<T>: IntoIterator,
{
  type IntoIter = std::vec::IntoIter<Self::Item>;
  type Item = <&'response SinglePageResponse<T> as IntoIterator>::Item;

  fn into_iter(self) -> Self::IntoIter {
    self
      .pages
      .iter()
      .flat_map(|page| page.into_iter().collect::<Vec<_>>())
      .collect::<Vec<_>>()
      .into_iter()
  }
}
