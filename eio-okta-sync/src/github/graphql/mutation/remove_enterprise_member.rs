#![allow(clippy::all, warnings)]
pub struct Mutation;
pub mod mutation {
  #![allow(dead_code)]
  use std::result::Result;
  pub const OPERATION_NAME: &str = "Mutation";
  pub const QUERY : & str = "mutation Mutation(\n  $clientMutationId: String,\n  $enterpriseId: ID!,\n  $userId: ID!,\n) {\n  removeEnterpriseMember(input: {\n    clientMutationId:$clientMutationId,\n    enterpriseId:$enterpriseId,\n    userId:$userId,\n  }) {\n    clientMutationId\n    enterprise {\n      databaseId\n      id\n      slug\n    }\n    user {\n      databaseId\n      email\n      id\n      login\n    }\n  }\n}\n" ;
  use super::*;
  use serde::{Deserialize, Serialize};
  #[allow(dead_code)]
  type Boolean = bool;
  #[allow(dead_code)]
  type Float = f64;
  #[allow(dead_code)]
  type Int = i64;
  #[allow(dead_code)]
  type ID = String;
  #[derive(Serialize, Debug, Clone, PartialEq, Eq, Hash, Deserialize, bon :: Builder)]
  #[serde(crate = ":: serde")]
  pub struct Variables {
    #[serde(rename = "clientMutationId")]
    pub client_mutation_id: Option<String>,
    #[serde(rename = "enterpriseId")]
    pub enterprise_id: ID,
    #[serde(rename = "userId")]
    pub user_id: ID,
  }
  impl Variables {}
  #[derive(Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
  #[serde(crate = ":: serde")]
  pub struct ResponseData {
    #[serde(rename = "removeEnterpriseMember")]
    pub remove_enterprise_member: Option<MutationRemoveEnterpriseMember>,
  }
  #[derive(Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
  #[serde(crate = ":: serde")]
  pub struct MutationRemoveEnterpriseMember {
    #[serde(rename = "clientMutationId")]
    pub client_mutation_id: Option<String>,
    pub enterprise: Option<MutationRemoveEnterpriseMemberEnterprise>,
    pub user: Option<MutationRemoveEnterpriseMemberUser>,
  }
  #[derive(Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
  #[serde(crate = ":: serde")]
  pub struct MutationRemoveEnterpriseMemberEnterprise {
    #[serde(rename = "databaseId")]
    pub database_id: Option<Int>,
    pub id: ID,
    pub slug: String,
  }
  #[derive(Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
  #[serde(crate = ":: serde")]
  pub struct MutationRemoveEnterpriseMemberUser {
    #[serde(rename = "databaseId")]
    pub database_id: Option<Int>,
    pub email: String,
    pub id: ID,
    pub login: String,
  }
}
impl graphql_client::GraphQLQuery for Mutation {
  type Variables = mutation::Variables;
  type ResponseData = mutation::ResponseData;
  fn build_query(variables: Self::Variables) -> graphql_client::QueryBody<Self::Variables> {
    graphql_client::QueryBody {
      variables,
      query: mutation::QUERY,
      operation_name: mutation::OPERATION_NAME,
    }
  }
}
