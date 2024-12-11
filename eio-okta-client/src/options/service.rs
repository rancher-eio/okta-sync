use clap::Args;
use eio_okta_api::traits::Service;
use http::uri::{Authority, Scheme};

#[derive(Debug, Clone, Args)]
#[group(skip)]
#[remain::sorted]
pub struct OktaService {
  #[arg(default_value = "suse.okta.com")]
  #[arg(env = "OKTA_SERVICE_AUTHORITY")]
  #[arg(id = "okta_service_authority")]
  #[arg(long = "okta-service-authority")]
  #[arg(value_name = "HOST[:PORT]")]
  pub authority: Authority,
  #[arg(default_value = "https")]
  #[arg(env = "OKTA_SERVICE_SCHEME")]
  #[arg(id = "okta_service_scheme")]
  #[arg(long = "okta-service-scheme")]
  #[arg(value_name = "SCHEME")]
  pub scheme: Scheme,
}

impl Service for OktaService {
  fn authority(&self) -> &Authority {
    &self.authority
  }

  fn scheme(&self) -> &Scheme {
    &self.scheme
  }
}
