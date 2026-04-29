#![allow(clippy::all, warnings)]
pub struct Mutation;
pub mod mutation {
  #![allow(dead_code)]
  use std::result::Result;
  pub const OPERATION_NAME: &str = "Mutation";
  pub const QUERY : & str = "mutation Mutation(\n  $clientMutationId: String,\n  $invitationId: ID!,\n) {\n  cancelEnterpriseMemberInvitation(input: {\n    clientMutationId: $clientMutationId,\n    invitationId: $invitationId,\n  }) {\n    clientMutationId\n    invitation {\n      email\n      enterprise {\n        databaseId\n        id\n        slug\n      }\n      id\n      invitee {\n        databaseId\n        email\n        id\n        login\n      }\n      inviter {\n        databaseId\n        email\n        id\n        login\n      }\n    }\n  }\n}\n" ;
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
    #[serde(rename = "invitationId")]
    pub invitation_id: ID,
  }
  impl Variables {}
  #[derive(Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
  #[serde(crate = ":: serde")]
  pub struct ResponseData {
    #[serde(rename = "cancelEnterpriseMemberInvitation")]
    pub cancel_enterprise_member_invitation: Option<MutationCancelEnterpriseMemberInvitation>,
  }
  #[derive(Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
  #[serde(crate = ":: serde")]
  pub struct MutationCancelEnterpriseMemberInvitation {
    #[serde(rename = "clientMutationId")]
    pub client_mutation_id: Option<String>,
    pub invitation: Option<MutationCancelEnterpriseMemberInvitationInvitation>,
  }
  #[derive(Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
  #[serde(crate = ":: serde")]
  pub struct MutationCancelEnterpriseMemberInvitationInvitation {
    pub email: Option<String>,
    pub enterprise: MutationCancelEnterpriseMemberInvitationInvitationEnterprise,
    pub id: ID,
    pub invitee: Option<MutationCancelEnterpriseMemberInvitationInvitationInvitee>,
    pub inviter: Option<MutationCancelEnterpriseMemberInvitationInvitationInviter>,
  }
  #[derive(Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
  #[serde(crate = ":: serde")]
  pub struct MutationCancelEnterpriseMemberInvitationInvitationEnterprise {
    #[serde(rename = "databaseId")]
    pub database_id: Option<Int>,
    pub id: ID,
    pub slug: String,
  }
  #[derive(Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
  #[serde(crate = ":: serde")]
  pub struct MutationCancelEnterpriseMemberInvitationInvitationInvitee {
    #[serde(rename = "databaseId")]
    pub database_id: Option<Int>,
    pub email: String,
    pub id: ID,
    pub login: String,
  }
  #[derive(Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
  #[serde(crate = ":: serde")]
  pub struct MutationCancelEnterpriseMemberInvitationInvitationInviter {
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
