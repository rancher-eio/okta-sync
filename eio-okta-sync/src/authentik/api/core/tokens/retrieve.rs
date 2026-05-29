use authentik_client::apis::Error;
use authentik_client::apis::configuration::Configuration;
use authentik_client::apis::core_api::{CoreTokensRetrieveError, core_tokens_retrieve};
use authentik_client::models::Token;

#[derive(Debug, Clone, PartialEq, Eq, clap::Parser, serde::Serialize, serde::Deserialize, bon::Builder)]
#[command(about = "Get information about a specific token")]
#[command(after_help = "API Reference @ https://api.goauthentik.io/reference/core-tokens-retrieve/")]
#[remain::sorted]
pub struct CoreTokensRetrieve {
  #[arg(help_heading = "Query Parameters")]
  #[arg(long)]
  #[arg(value_name = "STRING")]
  identifier: String,
}

impl crate::authentik::traits::GetWithConfiguration for CoreTokensRetrieve {
  type Error = Error<CoreTokensRetrieveError>;
  type Value = Token;

  async fn get_with_configuration(self, configuration: &Configuration) -> Result<Self::Value, Self::Error> {
    let Self { identifier } = self;

    core_tokens_retrieve(configuration, &identifier).await
  }
}

crate::authentik::macros::RunAsync!(CoreTokensRetrieve as Wrapper<GetWithConfiguration>);

crate::authentik::macros::From!(
  Error<CoreTokensRetrieveError>
  => super::CoreTokensApiError
  => super::super::CoreApiError
  => super::super::super::ApiError
  => super::super::super::super::Error
);
