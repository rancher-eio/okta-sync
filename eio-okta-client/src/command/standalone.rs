use crate::client::Client;
use crate::Options;
use clap::{Args, Parser};
use eio_okta_api::traits::{Endpoint, Pagination, Response};
use serde::de::DeserializeOwned;
use serde::Serialize;

#[derive(Debug, Parser)]
#[remain::sorted]
pub struct Standalone<T: Args + Serialize + DeserializeOwned> {
  #[command(flatten)]
  client_options: Options,
  #[command(flatten)]
  endpoint: T,
  #[arg(long)]
  #[arg(help_heading = "Output Options")]
  #[arg(help = "pretty-print JSON output?")]
  pretty: bool,
}

impl<T> Standalone<T>
where
  T: Endpoint + Serialize + DeserializeOwned + Args,
{
  pub fn resource(self) -> Result<<T as Response>::Body, crate::Error> {
    let Self {
      endpoint,
      client_options,
      ..
    } = self;

    let client = Client::from(client_options);
    client.call(endpoint)
  }

  pub fn resource_json(self) -> Result<String, crate::Error> {
    let pretty = self.pretty;
    let value = self.resource()?;
    let json = if pretty {
      serde_json::to_string_pretty(&value)
    } else {
      serde_json::to_string(&value)
    }?;

    Ok(json)
  }
}

impl<T> Standalone<T>
where
  T: Endpoint + Pagination + Serialize + DeserializeOwned + Args,
{
  pub fn resources(self) -> Result<Vec<<T as Pagination>::Item>, crate::Error> {
    let Self {
      endpoint,
      client_options,
      ..
    } = self;

    let client = Client::from(client_options);
    client.paginate(endpoint)
  }

  pub fn resources_json(self) -> Result<String, crate::Error> {
    let pretty = self.pretty;
    let value = self.resources()?;
    let json = if pretty {
      serde_json::to_string_pretty(&value)
    } else {
      serde_json::to_string(&value)
    }?;

    Ok(json)
  }
}
