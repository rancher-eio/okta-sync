use headers::{authorization::Credentials, Authorization};
use http::HeaderValue;
use std::str::FromStr;

#[derive(Clone)]
pub struct SSWS {
  token: HeaderValueString,
}

impl FromStr for SSWS {
  type Err = <HeaderValue as FromStr>::Err;
  fn from_str(src: &str) -> Result<Self, Self::Err> {
    let src = src.strip_prefix(<Self as Credentials>::SCHEME).unwrap_or(src).trim();

    let mut token = HeaderValueString::from_str(src)?;
    token.set_sensitive(true);

    Ok(Self { token })
  }
}

impl Credentials for SSWS {
  const SCHEME: &'static str = "SSWS";

  fn decode(value: &HeaderValue) -> Option<Self> {
    match value.to_str() {
      Ok(s) if s.is_ascii() && s.starts_with(Self::SCHEME) => Self::from_str(s).ok(),
      _ => None,
    }
  }

  fn encode(&self) -> HeaderValue {
    HeaderValue::from_str(&[Self::SCHEME, self.token.as_ref()].join(" ")).unwrap()
  }
}

impl From<SSWS> for Authorization<SSWS> {
  fn from(ssws: SSWS) -> Self {
    Self(ssws)
  }
}

#[derive(Clone)]
struct HeaderValueString(HeaderValue);

impl HeaderValueString {
  fn set_sensitive(&mut self, val: bool) {
    self.0.set_sensitive(val)
  }
}

impl FromStr for HeaderValueString {
  type Err = <HeaderValue as FromStr>::Err;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    HeaderValue::from_str(s).map(Self)
  }
}

impl TryFrom<HeaderValue> for HeaderValueString {
  type Error = http::header::ToStrError;
  fn try_from(value: HeaderValue) -> Result<Self, Self::Error> {
    value.to_str()?;
    Ok(Self(value))
  }
}

impl From<HeaderValueString> for HeaderValue {
  fn from(value: HeaderValueString) -> Self {
    value.0
  }
}

impl AsRef<str> for HeaderValueString {
  fn as_ref(&self) -> &str {
    self.0.to_str().unwrap()
  }
}
