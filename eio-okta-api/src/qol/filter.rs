use std::str::FromStr;

use clap::Args;
use serde::{Deserialize, Serialize};

use scim_proto::filter::ScimFilter;

#[derive(Debug, Clone, Args, Serialize, Deserialize, Default)]
pub struct Filter {
  #[arg(long, value_parser = parse_scim_filter_to_string)]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub filter: Option<String>,
}

fn parse_scim_filter_to_string(input: &str) -> Result<String, <ScimFilter as FromStr>::Err> {
  Ok(ScimFilter::from_str(input)?.to_string())
}
