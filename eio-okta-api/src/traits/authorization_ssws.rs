use crate::authorization::SSWS;
use std::str::FromStr;

pub trait AuthorizationSSWS: From<SSWS> {
  fn ssws(token: &str) -> Result<Self, <SSWS as FromStr>::Err> {
    SSWS::from_str(token).map(Into::into)
  }
}

impl AuthorizationSSWS for headers::Authorization<SSWS> {}
