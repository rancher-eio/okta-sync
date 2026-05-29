use std::ops::AddAssign;

use authentik_client::{apis::configuration::Configuration, models::Pagination};

pub trait Paginated {
  type Results: Send + IntoIterator + Extend<<Self::Results as IntoIterator>::Item>;

  fn pagination(&self) -> &Pagination;
  fn results(self) -> Self::Results;
}

pub trait PageMut {
  fn page_mut(&mut self) -> Option<&mut i32>;
}

pub trait GetWithConfiguration {
  type Error: std::error::Error + Into<crate::authentik::Error>;
  type Value;

  fn get_with_configuration(
    self,
    configuration: &Configuration,
  ) -> impl Future<Output = Result<Self::Value, Self::Error>> + Send;
}

pub trait AutopaginateWithConfiguration: GetWithConfiguration
where
  Self::Value: Paginated,
{
  fn autopaginate_with_configuration(
    self,
    configuration: &Configuration,
  ) -> impl Future<Output = Result<<Self::Value as Paginated>::Results, Self::Error>> + Send;
}

impl<T> AutopaginateWithConfiguration for T
where
  T: GetWithConfiguration + Clone + PageMut + Send,
  T::Value: Paginated + Send,
{
  async fn autopaginate_with_configuration(
    mut self,
    configuration: &Configuration,
  ) -> Result<<Self::Value as Paginated>::Results, Self::Error> {
    let page = self.clone().get_with_configuration(configuration).await?;
    let mut next = page.pagination().next > 0.0;

    let mut results = page.results();

    while next {
      self.page_mut().get_or_insert(&mut 0).add_assign(1);
      let page = self.clone().get_with_configuration(configuration).await?;
      next = page.pagination().next > 0.0;
      results.extend(page.results());
    }

    Ok(results)
  }
}

pub(crate) trait RunAsync {
  async fn run(self) -> Result<(), crate::authentik::Error>;
}
