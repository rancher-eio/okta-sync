#![allow(clippy::all, warnings)]
pub struct Query;
pub mod query {
  #![allow(dead_code)]
  use std::result::Result;
  pub const OPERATION_NAME: &str = "Query";
  pub const QUERY: &str = "query Query(\n  $enterpriseSlug:String!,\n  $first:Int=100,\n  $after:String,\n) {\n  enterprise(slug:$enterpriseSlug) {\n    ownerInfo {\n      pendingCollaboratorInvitations(first:$first,after:$after) {\n        pageInfo {\n          endCursor\n          hasNextPage\n        }\n        nodes {\n          email\n          id\n          invitee {\n            databaseId\n            email\n            id\n            login\n          }\n          inviter {\n            databaseId\n            email\n            id\n            login\n          }\n          permission\n          repository {\n            __typename\n            ... on Repository {\n              databaseId\n              id\n              name\n              owner {\n                __typename\n                ... on Organization {\n                  databaseId\n                  id\n                  login\n                }\n                ... on User {\n                  databaseId\n                  email\n                  id\n                  login\n                }\n              }\n              visibility\n            }\n          }\n        }\n      }\n    }\n  }\n}\n";
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
  #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
  pub enum RepositoryPermission {
    ADMIN,
    MAINTAIN,
    READ,
    TRIAGE,
    WRITE,
    Other(String),
  }
  impl ::serde::Serialize for RepositoryPermission {
    fn serialize<S: ::serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
      ser.serialize_str(match *self {
        RepositoryPermission::ADMIN => "ADMIN",
        RepositoryPermission::MAINTAIN => "MAINTAIN",
        RepositoryPermission::READ => "READ",
        RepositoryPermission::TRIAGE => "TRIAGE",
        RepositoryPermission::WRITE => "WRITE",
        RepositoryPermission::Other(ref s) => &s,
      })
    }
  }
  impl<'de> ::serde::Deserialize<'de> for RepositoryPermission {
    fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
      let s: String = ::serde::Deserialize::deserialize(deserializer)?;
      match s.as_str() {
        "ADMIN" => Ok(RepositoryPermission::ADMIN),
        "MAINTAIN" => Ok(RepositoryPermission::MAINTAIN),
        "READ" => Ok(RepositoryPermission::READ),
        "TRIAGE" => Ok(RepositoryPermission::TRIAGE),
        "WRITE" => Ok(RepositoryPermission::WRITE),
        _ => Ok(RepositoryPermission::Other(s)),
      }
    }
  }
  #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
  pub enum RepositoryVisibility {
    INTERNAL,
    PRIVATE,
    PUBLIC,
    Other(String),
  }
  impl ::serde::Serialize for RepositoryVisibility {
    fn serialize<S: ::serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
      ser.serialize_str(match *self {
        RepositoryVisibility::INTERNAL => "INTERNAL",
        RepositoryVisibility::PRIVATE => "PRIVATE",
        RepositoryVisibility::PUBLIC => "PUBLIC",
        RepositoryVisibility::Other(ref s) => &s,
      })
    }
  }
  impl<'de> ::serde::Deserialize<'de> for RepositoryVisibility {
    fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
      let s: String = ::serde::Deserialize::deserialize(deserializer)?;
      match s.as_str() {
        "INTERNAL" => Ok(RepositoryVisibility::INTERNAL),
        "PRIVATE" => Ok(RepositoryVisibility::PRIVATE),
        "PUBLIC" => Ok(RepositoryVisibility::PUBLIC),
        _ => Ok(RepositoryVisibility::Other(s)),
      }
    }
  }
  #[derive(Serialize, Debug, Clone, PartialEq, Eq, Hash, Deserialize, bon :: Builder)]
  #[serde(crate = ":: serde")]
  pub struct Variables {
    #[serde(rename = "enterpriseSlug")]
    pub enterprise_slug: String,
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
    #[serde(rename = "pendingCollaboratorInvitations")]
    pub pending_collaborator_invitations: QueryEnterpriseOwnerInfoPendingCollaboratorInvitations,
  }
  #[derive(Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
  #[serde(crate = ":: serde")]
  pub struct QueryEnterpriseOwnerInfoPendingCollaboratorInvitations {
    #[serde(rename = "pageInfo")]
    pub page_info: QueryEnterpriseOwnerInfoPendingCollaboratorInvitationsPageInfo,
    pub nodes: Option<Vec<Option<QueryEnterpriseOwnerInfoPendingCollaboratorInvitationsNodes>>>,
  }
  #[derive(Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
  #[serde(crate = ":: serde")]
  pub struct QueryEnterpriseOwnerInfoPendingCollaboratorInvitationsPageInfo {
    #[serde(rename = "endCursor")]
    pub end_cursor: Option<String>,
    #[serde(rename = "hasNextPage")]
    pub has_next_page: Boolean,
  }
  #[derive(Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
  #[serde(crate = ":: serde")]
  pub struct QueryEnterpriseOwnerInfoPendingCollaboratorInvitationsNodes {
    pub email: Option<String>,
    pub id: ID,
    pub invitee: Option<QueryEnterpriseOwnerInfoPendingCollaboratorInvitationsNodesInvitee>,
    pub inviter: QueryEnterpriseOwnerInfoPendingCollaboratorInvitationsNodesInviter,
    pub permission: RepositoryPermission,
    pub repository: Option<QueryEnterpriseOwnerInfoPendingCollaboratorInvitationsNodesRepository>,
  }
  #[derive(Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
  #[serde(crate = ":: serde")]
  pub struct QueryEnterpriseOwnerInfoPendingCollaboratorInvitationsNodesInvitee {
    #[serde(rename = "databaseId")]
    pub database_id: Option<Int>,
    pub email: String,
    pub id: ID,
    pub login: String,
  }
  #[derive(Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
  #[serde(crate = ":: serde")]
  pub struct QueryEnterpriseOwnerInfoPendingCollaboratorInvitationsNodesInviter {
    #[serde(rename = "databaseId")]
    pub database_id: Option<Int>,
    pub email: String,
    pub id: ID,
    pub login: String,
  }
  #[derive(Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
  #[serde(tag = "__typename")]
  pub enum QueryEnterpriseOwnerInfoPendingCollaboratorInvitationsNodesRepository {
    Repository(QueryEnterpriseOwnerInfoPendingCollaboratorInvitationsNodesRepositoryOnRepository),
  }
  #[derive(Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
  #[serde(crate = ":: serde")]
  pub struct QueryEnterpriseOwnerInfoPendingCollaboratorInvitationsNodesRepositoryOnRepository {
    #[serde(rename = "databaseId")]
    pub database_id: Option<Int>,
    pub id: ID,
    pub name: String,
    pub owner: QueryEnterpriseOwnerInfoPendingCollaboratorInvitationsNodesRepositoryOnRepositoryOwner,
    pub visibility: RepositoryVisibility,
  }
  #[derive(Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
  #[serde(tag = "__typename")]
  pub enum QueryEnterpriseOwnerInfoPendingCollaboratorInvitationsNodesRepositoryOnRepositoryOwner {
    Organization(QueryEnterpriseOwnerInfoPendingCollaboratorInvitationsNodesRepositoryOnRepositoryOwnerOnOrganization),
    User(QueryEnterpriseOwnerInfoPendingCollaboratorInvitationsNodesRepositoryOnRepositoryOwnerOnUser),
  }
  #[derive(Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
  #[serde(crate = ":: serde")]
  pub struct QueryEnterpriseOwnerInfoPendingCollaboratorInvitationsNodesRepositoryOnRepositoryOwnerOnOrganization {
    #[serde(rename = "databaseId")]
    pub database_id: Option<Int>,
    pub id: ID,
    pub login: String,
  }
  #[derive(Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
  #[serde(crate = ":: serde")]
  pub struct QueryEnterpriseOwnerInfoPendingCollaboratorInvitationsNodesRepositoryOnRepositoryOwnerOnUser {
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
