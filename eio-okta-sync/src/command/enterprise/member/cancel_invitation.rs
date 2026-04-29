use octocrab::Octocrab;

use crate::github::graphql::mutation::cancel_enterprise_member_invitation::Mutation;
use crate::graphql::{ExecuteQuery, Variables};

#[derive(Debug, clap::Parser)]
#[remain::sorted]
#[rustfmt::skip]
#[command(about = "cancels a pending invitation for an unaffiliated member to join an enterprise")]
pub struct Command {
  #[arg(
    long,
    value_name = "STRING",
    help       = "a unique identifier for the client performing the mutation"
  )]
  client_mutation_id: Option<String>,
  #[arg(
    long,
    value_name = "ID",
    help = "the invitation ID of the pending enterprise member",
  )]
  invitation_id: String,
  #[command(flatten)]
  token: crate::args::github::Token,
}

impl Command {
  #[allow(unused_variables)]
  pub async fn run(self) -> Result<(), crate::Error> {
    let Self {
      client_mutation_id,
      invitation_id,
      token,
    } = self;

    let github = Octocrab::builder().personal_token(token).build()?;

    let variables = Mutation::variables()
      .invitation_id(invitation_id)
      .maybe_client_mutation_id(client_mutation_id)
      .build();
    let response = Mutation::execute_query(variables, &github).await?;

    let json = serde_json::to_string_pretty(&response)?;

    println!("{json}");

    Ok(())
  }
}
