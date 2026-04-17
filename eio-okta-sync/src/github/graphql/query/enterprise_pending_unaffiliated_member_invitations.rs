#![allow(clippy::all, warnings)]
pub struct Query;
pub mod query {
  #![allow(dead_code)]
  use std::result::Result;
  pub const OPERATION_NAME: &str = "Query";
  pub const QUERY : & str = "query Query(\n  $enterprise:String!,\n  $first:Int=100,\n  $after:String,\n) {\n  enterprise(slug:$enterprise) {\n    ownerInfo {\n      pendingUnaffiliatedMemberInvitations(first:$first,after:$after) {\n        pageInfo {\n          endCursor\n          hasNextPage\n        }\n        nodes {\n          email\n          enterprise {\n            databaseId\n            id\n            slug\n          }\n          id\n          invitee {\n            databaseId\n            email\n            id\n            login\n          }\n          inviter {\n            databaseId\n            email\n            id\n            login\n          }\n        }\n      }\n    }\n  }\n}\n" ;
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
    #[serde(rename = "pendingUnaffiliatedMemberInvitations")]
    pub pending_unaffiliated_member_invitations: QueryEnterpriseOwnerInfoPendingUnaffiliatedMemberInvitations,
  }
  #[derive(Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
  #[serde(crate = ":: serde")]
  pub struct QueryEnterpriseOwnerInfoPendingUnaffiliatedMemberInvitations {
    #[serde(rename = "pageInfo")]
    pub page_info: QueryEnterpriseOwnerInfoPendingUnaffiliatedMemberInvitationsPageInfo,
    pub nodes: Option<Vec<Option<QueryEnterpriseOwnerInfoPendingUnaffiliatedMemberInvitationsNodes>>>,
  }
  #[derive(Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
  #[serde(crate = ":: serde")]
  pub struct QueryEnterpriseOwnerInfoPendingUnaffiliatedMemberInvitationsPageInfo {
    #[serde(rename = "endCursor")]
    pub end_cursor: Option<String>,
    #[serde(rename = "hasNextPage")]
    pub has_next_page: Boolean,
  }
  #[derive(Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
  #[serde(crate = ":: serde")]
  pub struct QueryEnterpriseOwnerInfoPendingUnaffiliatedMemberInvitationsNodes {
    pub email: Option<String>,
    pub enterprise: QueryEnterpriseOwnerInfoPendingUnaffiliatedMemberInvitationsNodesEnterprise,
    pub id: ID,
    pub invitee: Option<QueryEnterpriseOwnerInfoPendingUnaffiliatedMemberInvitationsNodesInvitee>,
    pub inviter: Option<QueryEnterpriseOwnerInfoPendingUnaffiliatedMemberInvitationsNodesInviter>,
  }
  #[derive(Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
  #[serde(crate = ":: serde")]
  pub struct QueryEnterpriseOwnerInfoPendingUnaffiliatedMemberInvitationsNodesEnterprise {
    #[serde(rename = "databaseId")]
    pub database_id: Option<Int>,
    pub id: ID,
    pub slug: String,
  }
  #[derive(Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
  #[serde(crate = ":: serde")]
  pub struct QueryEnterpriseOwnerInfoPendingUnaffiliatedMemberInvitationsNodesInvitee {
    #[serde(rename = "databaseId")]
    pub database_id: Option<Int>,
    pub email: String,
    pub id: ID,
    pub login: String,
  }
  #[derive(Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
  #[serde(crate = ":: serde")]
  pub struct QueryEnterpriseOwnerInfoPendingUnaffiliatedMemberInvitationsNodesInviter {
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
