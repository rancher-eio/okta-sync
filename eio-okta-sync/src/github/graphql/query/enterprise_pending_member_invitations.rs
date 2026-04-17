#![allow(clippy::all, warnings)]
pub struct Query;
pub mod query {
  #![allow(dead_code)]
  use std::result::Result;
  pub const OPERATION_NAME: &str = "Query";
  pub const QUERY : & str = "query Query(\n  $enterprise:String!,\n  $first:Int=100,\n  $after:String,\n) {\n  enterprise(slug:$enterprise) {\n    ownerInfo {\n      pendingMemberInvitations(first:$first,after:$after) {\n        pageInfo {\n          endCursor\n          hasNextPage\n        }\n        nodes {\n          email\n          id\n          invitee {\n            databaseId\n            email\n            id\n            login\n          }\n          inviterActor {\n            databaseId\n            email\n            id\n            login\n          }\n          organization {\n            databaseId\n            id\n            login\n          }\n        }\n      }\n    }\n  }\n}" ;
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
    pub enterprise: String,
    pub first: Option<Int>,
    pub after: Option<String>,
  }
  impl Variables {
    pub fn default_first() -> Option<Int> {
      Some(100i64)
    }
  }
  #[derive(Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
  #[serde(crate = ":: serde")]
  pub struct ResponseData {
    pub enterprise: Option<QueryEnterprise>,
  }
  #[derive(Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
  #[serde(crate = ":: serde")]
  pub struct QueryEnterprise {
    #[serde(rename = "ownerInfo")]
    pub owner_info: Option<QueryEnterpriseOwnerInfo>,
  }
  #[derive(Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
  #[serde(crate = ":: serde")]
  pub struct QueryEnterpriseOwnerInfo {
    #[serde(rename = "pendingMemberInvitations")]
    pub pending_member_invitations: QueryEnterpriseOwnerInfoPendingMemberInvitations,
  }
  #[derive(Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
  #[serde(crate = ":: serde")]
  pub struct QueryEnterpriseOwnerInfoPendingMemberInvitations {
    #[serde(rename = "pageInfo")]
    pub page_info: QueryEnterpriseOwnerInfoPendingMemberInvitationsPageInfo,
    pub nodes: Option<Vec<Option<QueryEnterpriseOwnerInfoPendingMemberInvitationsNodes>>>,
  }
  #[derive(Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
  #[serde(crate = ":: serde")]
  pub struct QueryEnterpriseOwnerInfoPendingMemberInvitationsPageInfo {
    #[serde(rename = "endCursor")]
    pub end_cursor: Option<String>,
    #[serde(rename = "hasNextPage")]
    pub has_next_page: Boolean,
  }
  #[derive(Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
  #[serde(crate = ":: serde")]
  pub struct QueryEnterpriseOwnerInfoPendingMemberInvitationsNodes {
    pub email: Option<String>,
    pub id: ID,
    pub invitee: Option<QueryEnterpriseOwnerInfoPendingMemberInvitationsNodesInvitee>,
    #[serde(rename = "inviterActor")]
    pub inviter_actor: Option<QueryEnterpriseOwnerInfoPendingMemberInvitationsNodesInviterActor>,
    pub organization: QueryEnterpriseOwnerInfoPendingMemberInvitationsNodesOrganization,
  }
  #[derive(Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
  #[serde(crate = ":: serde")]
  pub struct QueryEnterpriseOwnerInfoPendingMemberInvitationsNodesInvitee {
    #[serde(rename = "databaseId")]
    pub database_id: Option<Int>,
    pub email: String,
    pub id: ID,
    pub login: String,
  }
  #[derive(Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
  #[serde(crate = ":: serde")]
  pub struct QueryEnterpriseOwnerInfoPendingMemberInvitationsNodesInviterActor {
    #[serde(rename = "databaseId")]
    pub database_id: Option<Int>,
    pub email: String,
    pub id: ID,
    pub login: String,
  }
  #[derive(Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
  #[serde(crate = ":: serde")]
  pub struct QueryEnterpriseOwnerInfoPendingMemberInvitationsNodesOrganization {
    #[serde(rename = "databaseId")]
    pub database_id: Option<Int>,
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
