#![allow(clippy::all, warnings)]
pub struct Mutation;
pub mod mutation {
  #![allow(dead_code)]
  use std::result::Result;
  pub const OPERATION_NAME: &str = "Mutation";
  pub const QUERY : & str = "mutation Mutation(\n  $clientMutationId: String,\n  $email: String,\n  $enterpriseId: ID!,\n  $invitee: String,\n) {\n  inviteEnterpriseMember(input: {\n    clientMutationId: $clientMutationId,\n    email: $email,\n    enterpriseId: $enterpriseId,\n    invitee: $invitee,\n  }) {\n    clientMutationId\n    invitation {\n      email\n      id\n      invitee {\n        databaseId\n        email\n        id\n        login\n      }\n      inviter {\n        databaseId\n        email\n        id\n        login\n      }\n    }\n  }\n}" ;
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
    pub email: Option<String>,
    #[serde(rename = "enterpriseId")]
    pub enterprise_id: ID,
    pub invitee: Option<String>,
  }
  impl Variables {}
  #[derive(Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
  #[serde(crate = ":: serde")]
  pub struct ResponseData {
    #[serde(rename = "inviteEnterpriseMember")]
    pub invite_enterprise_member: Option<MutationInviteEnterpriseMember>,
  }
  #[derive(Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
  #[serde(crate = ":: serde")]
  pub struct MutationInviteEnterpriseMember {
    #[serde(rename = "clientMutationId")]
    pub client_mutation_id: Option<String>,
    pub invitation: Option<MutationInviteEnterpriseMemberInvitation>,
  }
  #[derive(Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
  #[serde(crate = ":: serde")]
  pub struct MutationInviteEnterpriseMemberInvitation {
    pub email: Option<String>,
    pub id: ID,
    pub invitee: Option<MutationInviteEnterpriseMemberInvitationInvitee>,
    pub inviter: Option<MutationInviteEnterpriseMemberInvitationInviter>,
  }
  #[derive(Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
  #[serde(crate = ":: serde")]
  pub struct MutationInviteEnterpriseMemberInvitationInvitee {
    #[serde(rename = "databaseId")]
    pub database_id: Option<Int>,
    pub email: String,
    pub id: ID,
    pub login: String,
  }
  #[derive(Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
  #[serde(crate = ":: serde")]
  pub struct MutationInviteEnterpriseMemberInvitationInviter {
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
