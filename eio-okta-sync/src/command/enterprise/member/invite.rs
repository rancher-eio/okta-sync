use std::str::FromStr;

use graphql_client::{GraphQLQuery, Response};
use octocrab::Octocrab;

#[derive(Debug, clap::Parser)]
#[remain::sorted]
#[rustfmt::skip]
#[command(about = "assign members to an Enterprise Team based on Okta snapshot data")]
pub struct Command {
  #[arg(
    long,
    value_name = "STRING",
    help       = "a unique identifier for the client performing the mutation"
  )]
  client_mutation_id: Option<String>,
  #[arg(
    long,
    value_name    = "SLUG",
    default_value = "suse-gmbh",
    help          = "name of enterprise to use"
  )]
  enterprise: String,
  #[arg(
    long       = "invitee",
    value_name = "(USERNAME|EMAIL)",
    help       = "who to invite",
  )]
  invitee: Invitee,
  #[arg(
    env        = "GITHUB_TOKEN",
    long       = "token",
    value_name = "TOKEN",
    help       = "GitHub Access Token",
  )]
  token: String,
}

#[derive(Debug, Clone)]
enum Invitee {
  Email(String),
  Login(String),
}

impl FromStr for Invitee {
  type Err = std::convert::Infallible;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let invitee = String::from(s);
    if invitee.contains('@') {
      Ok(Self::Email(invitee))
    } else {
      Ok(Self::Login(invitee))
    }
  }
}

impl Command {
  #[allow(unused_variables)]
  pub async fn run(self) -> Result<(), crate::Error> {
    let Self {
      client_mutation_id,
      enterprise,
      token,
      invitee,
    } = self;

    let github = Octocrab::builder().personal_token(token).build()?;

    let enterprise_id = {
      use crate::github::graphql::query::enterprise::Query;
      let payload = <Query as GraphQLQuery>::Variables::builder()
        .enterprise(enterprise)
        .build();
      let response = github
        .graphql::<Response<<Query as GraphQLQuery>::ResponseData>>(&payload)
        .await?;
      response.data.unwrap().enterprise.unwrap().id
    };

    let response = {
      use crate::github::graphql::mutation::invite_enterprise_member::Mutation;

      let variables = {
        let builder = <Mutation as GraphQLQuery>::Variables::builder()
          .enterprise_id(enterprise_id)
          .maybe_client_mutation_id(client_mutation_id);

        match invitee {
          Invitee::Email(email) => builder.email(email).build(),
          Invitee::Login(login) => builder.invitee(login).build(),
        }
      };

      let payload = Mutation::build_query(variables);
      github
        .graphql::<Response<<Mutation as GraphQLQuery>::ResponseData>>(&payload)
        .await?
    };

    let json = serde_json::to_string_pretty(&response)?;

    println!("{json}");

    Ok(())
  }
}
