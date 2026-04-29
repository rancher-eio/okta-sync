use std::str::FromStr;

use crate::github::graphql::mutation::invite_enterprise_member::Mutation;
use crate::graphql::{ExecuteQuery, Variables};

#[derive(Debug, clap::Parser)]
#[remain::sorted]
#[rustfmt::skip]
#[command(about = "invite someone to become an unaffiliated member of the enterprise")]
pub struct Command {
  #[arg(
    long,
    value_name = "STRING",
    help       = "a unique identifier for the client performing the mutation"
  )]
  client_mutation_id: Option<String>,
  #[command(flatten)]
  enterprise: crate::args::github::EnterpriseIdOrSlug,
  #[arg(
    long       = "invitee",
    value_name = "(USERNAME|EMAIL)",
    help       = "who to invite",
  )]
  invitee: Invitee,
  #[command(flatten)]
  token: crate::args::github::Token,
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

    let client = token.try_into()?;

    let enterprise_id = enterprise.id(&client).await?;

    let variables = {
      let builder = Mutation::variables()
        .enterprise_id(enterprise_id)
        .maybe_client_mutation_id(client_mutation_id);

      match invitee {
        Invitee::Email(email) => builder.email(email).build(),
        Invitee::Login(login) => builder.invitee(login).build(),
      }
    };

    let response = Mutation::execute_query(variables, &client).await?;

    let json = serde_json::to_string_pretty(&response)?;

    println!("{json}");

    Ok(())
  }
}
