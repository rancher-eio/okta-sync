// convenience hack
// mostly because the generated authentik_client api code generally expects Option<&str>,
// but it's tedious to wrangle potentially dozens of lifetime parameters with builders.
// like... struct RequestBuilder<'_, '_, '_, '_, ...> 🤮

use email_address::EmailAddress;

pub(crate) trait MapAsStr {
  fn map_as_str(&self) -> Option<&str>;
}

impl MapAsStr for Option<String> {
  fn map_as_str(&self) -> Option<&str> {
    self.as_ref().map(String::as_str)
  }
}

impl MapAsStr for Option<EmailAddress> {
  fn map_as_str(&self) -> Option<&str> {
    self.as_ref().map(EmailAddress::as_str)
  }
}
