use graphql_client::GraphQLQuery;
use octocrab::Octocrab;

use crate::graphql::*;

pub(crate) mod mutation;
pub(crate) mod query;

impl OnePage for Octocrab {
  async fn graphql_one_page<T: GraphQLQuery>(
    &self,
    variables: <T as GraphQLQuery>::Variables,
  ) -> Result<SinglePageResponse<<T as GraphQLQuery>::ResponseData>, crate::Error> {
    let payload = T::build_query(variables);
    let response = self.graphql(&payload).await?;

    Ok(response)
  }
}
