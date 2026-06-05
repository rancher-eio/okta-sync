use std::str::FromStr;

use clap::{Args, Parser};

use authentik_client::apis::configuration::Configuration;
use http::uri::{Authority, Scheme};
use iri_string::types::{UriAbsoluteString, UriRelativeString};
use secrecy::{ExposeSecret, SecretString};

#[derive(Debug, Clone, Parser)]
#[clap(rename_all = "kebab-case")]
#[remain::sorted]
pub(crate) struct AuthentikOptions {
  #[command(flatten)]
  service: AuthentikServiceOptions,
  #[arg(long = "authentik-token")]
  #[arg(env = "AUTHENTIK_TOKEN")]
  #[arg(help_heading = "API Credentials")]
  #[arg(hide_env_values = true)]
  token: SecretString,
}

#[derive(Debug, Clone, Args)]
#[group(skip)]
pub(crate) struct AuthentikServiceOptions {
  #[arg(default_value = "id.suse.com")]
  #[arg(env = "AUTHENTIK_SERVICE_AUTHORITY")]
  #[arg(help = "Host and (optional) Port for Authentik Service")]
  #[arg(help_heading = "Authentik Service Options")]
  #[arg(long = "authentik-service-authority")]
  #[arg(value_name = "HOST[:PORT]")]
  authority: Authority,
  #[arg(default_value = "/api/v3")]
  #[arg(env = "AUTHENTIK_SERVICE_BASE_PATH")]
  #[arg(help_heading = "Authentik Service Options")]
  #[arg(help = "Base API Path for Authentik Service")]
  #[arg(long = "authentik-service-base-path")]
  #[arg(value_name = "RELATIVE-URI")]
  base_path: UriRelativeString,
  #[arg(default_value = "https")]
  #[arg(env = "AUTHENTIK_SERVICE_SCHEME")]
  #[arg(help = "Protocol Scheme for Authentik Service")]
  #[arg(help_heading = "Authentik Service Options")]
  #[arg(long = "authentik-service-scheme")]
  #[arg(value_name = "SCHEME")]
  scheme: Scheme,
}

#[nutype::nutype(derive(Debug, Clone, From, Into, Deref, Display))]
pub(crate) struct AuthentikService(UriAbsoluteString);

impl From<AuthentikServiceOptions> for AuthentikService {
  fn from(
    AuthentikServiceOptions {
      authority,
      base_path,
      scheme,
    }: AuthentikServiceOptions,
  ) -> Self {
    UriAbsoluteString::from_str(
      &http::Uri::builder()
        .scheme(scheme)
        .authority(authority)
        .path_and_query(base_path.as_str())
        .build()
        .expect("(╯°□°)╯︵ ┻━┻")
        .to_string(),
    )
    .expect("(╯°□°)╯︵ ┻━┻")
    .into()
  }
}

impl From<AuthentikOptions> for Configuration {
  fn from(AuthentikOptions { service, token }: AuthentikOptions) -> Self {
    Self {
      base_path: AuthentikService::from(service).to_string(),
      bearer_access_token: Some(token.expose_secret().to_string()),
      ..Default::default()
    }
  }
}
