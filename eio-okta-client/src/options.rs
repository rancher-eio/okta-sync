mod agent;
mod service;

pub use agent::UreqAgent;
pub use service::OktaService;

use clap::Args;
use educe::Educe;
use eio_okta_api::{authorization::SSWS, traits::AuthorizationSSWS};
use headers::Authorization;

#[derive(Educe, Clone, Args)]
#[educe(Debug)]
#[group(skip)]
pub struct Options {
  #[command(flatten)]
  pub agent: UreqAgent,
  #[educe(Debug(ignore))]
  #[arg(long = "authorization", env = "OKTA_AUTHORIZATION", value_name = "SSWS-TOKEN", value_parser = <Authorization<SSWS> as AuthorizationSSWS>::ssws)]
  pub authorization: Authorization<SSWS>,
  #[arg(long, help = "automatically fetch additional results when available?")]
  pub auto_paginate: bool,
  #[command(flatten)]
  pub service: OktaService,
}
