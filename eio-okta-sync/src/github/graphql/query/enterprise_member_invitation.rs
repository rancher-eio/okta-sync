#![allow(clippy::all, warnings)]
pub struct Query;
pub mod query {
  #![allow(dead_code)]
  use std::result::Result;
  pub const OPERATION_NAME: &str = "Query";
  pub const QUERY : & str = "query Query(\n  $enterpriseSlug: String!,\n  $userLogin: String!,\n) {\n  enterpriseMemberInvitation(\n    enterpriseSlug: $enterpriseSlug,\n    userLogin: $userLogin,\n  ) {\n    email\n    enterprise {\n      databaseId\n      id\n      slug\n    }\n    id\n    invitee {\n      databaseId\n      email\n      id\n      login\n    }\n    inviter {\n      databaseId\n      email\n      id\n      login\n    }\n  }\n}\n" ;
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
    #[serde(rename = "enterpriseSlug")]
    pub enterprise_slug: String,
    #[serde(rename = "userLogin")]
    pub user_login: String,
  }
  impl Variables {}
  #[derive(Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
  #[serde(crate = ":: serde")]
  pub struct ResponseData {
    #[serde(rename = "enterpriseMemberInvitation")]
    pub enterprise_member_invitation: Option<QueryEnterpriseMemberInvitation>,
  }
  #[derive(Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
  #[serde(crate = ":: serde")]
  pub struct QueryEnterpriseMemberInvitation {
    pub email: Option<String>,
    pub enterprise: QueryEnterpriseMemberInvitationEnterprise,
    pub id: ID,
    pub invitee: Option<QueryEnterpriseMemberInvitationInvitee>,
    pub inviter: Option<QueryEnterpriseMemberInvitationInviter>,
  }
  #[derive(Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
  #[serde(crate = ":: serde")]
  pub struct QueryEnterpriseMemberInvitationEnterprise {
    #[serde(rename = "databaseId")]
    pub database_id: Option<Int>,
    pub id: ID,
    pub slug: String,
  }
  #[derive(Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
  #[serde(crate = ":: serde")]
  pub struct QueryEnterpriseMemberInvitationInvitee {
    #[serde(rename = "databaseId")]
    pub database_id: Option<Int>,
    pub email: String,
    pub id: ID,
    pub login: String,
  }
  #[derive(Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
  #[serde(crate = ":: serde")]
  pub struct QueryEnterpriseMemberInvitationInviter {
    #[serde(rename = "databaseId")]
    pub database_id: Option<Int>,
    pub email: String,
    pub id: ID,
    pub login: String,
  }
}
impl graphql_client::GraphQLQuery for Query {
  type Variables = query::Variables;
  type ResponseData = query::ResponseData;
  fn build_query(variables: Self::Variables) -> graphql_client::QueryBody<Self::Variables> {
    graphql_client::QueryBody {
      variables,
      query: query::QUERY,
      operation_name: query::OPERATION_NAME,
    }
  }
}
