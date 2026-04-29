use graphql_client::GraphQLQuery;

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
  pub errors: Option<Vec<graphql_client::Error>>,
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

pub(crate) trait Builder {
  type Builder;

  fn builder() -> Self::Builder;
}

pub(crate) trait Variables: GraphQLQuery
where
  Self::Variables: Builder,
{
  fn variables() -> <Self::Variables as Builder>::Builder {
    Self::Variables::builder()
  }
}

impl<T: GraphQLQuery> Variables for T where T::Variables: Builder {}

pub(crate) trait ExecuteQuery: GraphQLQuery {
  async fn execute_query(
    variables: Self::Variables,
    client: &octocrab::Octocrab,
  ) -> octocrab::Result<SinglePageResponse<Self::ResponseData>> {
    let payload = Self::build_query(variables);
    client.graphql(&payload).await
  }
}

impl<T: GraphQLQuery> ExecuteQuery for T {}

pub(crate) trait Direction {}

pub(crate) enum Forward {}

impl Direction for Forward {}

pub(crate) trait PaginateQuery<D: Direction = Forward>: ExecuteQuery
where
  Self::ResponseData: PageInfo,
{
  async fn paginate_query(
    variables: Self::Variables,
    client: &octocrab::Octocrab,
  ) -> octocrab::Result<MultiPageResponse<Self::ResponseData>>;
}

impl<T> PaginateQuery<Forward> for T
where
  T: GraphQLQuery + ExecuteQuery + PageSize,
  T::ResponseData: PageInfo,
  <T::ResponseData as PageInfo>::PageInfo: ForwardPageInfo,
  T::Variables: ForwardPageVariables + Clone,
{
  async fn paginate_query(
    mut variables: Self::Variables,
    client: &octocrab::Octocrab,
  ) -> octocrab::Result<MultiPageResponse<Self::ResponseData>> {
    T::enforce(variables.first());

    let mut pages = Vec::new();
    let mut next = true;

    while next {
      let response = Self::execute_query(variables.clone(), client).await?;
      next = T::page_forward(&mut variables, &response.data);
      pages.push(response);
    }

    Ok(MultiPageResponse { pages })
  }
}
