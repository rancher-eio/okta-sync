use octocrab::Octocrab;

use crate::github::graphql::mutation::remove_enterprise_member::Mutation;
use crate::graphql::{ExecuteQuery, Variables};

#[derive(Debug, clap::Parser)]
#[remain::sorted]
#[rustfmt::skip]
#[command(about = "completely removes a user from the enterprise")]
pub struct Command {
  #[arg(
    long,
    value_name = "STRING",
    help       = "a unique identifier for the client performing the mutation"
  )]
  client_mutation_id: Option<String>,
  #[command(flatten)]
  enterprise: crate::args::github::EnterpriseIdOrSlug,
  #[command(flatten)]
  token: crate::args::github::Token,
  #[command(flatten)]
  user: crate::args::github::UserIdOrLogin,
}

impl Command {
  #[allow(unused_variables)]
  pub async fn run(self) -> Result<(), crate::Error> {
    let Self {
      client_mutation_id,
      enterprise,
      token,
      user,
    } = self;

    let github = Octocrab::builder().personal_token(token).build()?;

    let user_id = user.id(&github).await?;
    let enterprise_id = enterprise.id(&github).await?;

    let response = {
      let variables = Mutation::variables()
        .enterprise_id(enterprise_id)
        .user_id(user_id)
        .maybe_client_mutation_id(client_mutation_id)
        .build();

      Mutation::execute_query(variables, &github).await?
    };

    let json = serde_json::to_string_pretty(&response)?;

    println!("{json}");

    Ok(())
  }
}
