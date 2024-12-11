#[crate::apply(crate::enums!)]
#[derive(Copy)]
#[cfg_attr(feature = "serde", serde(rename_all = "SCREAMING_SNAKE_CASE"))]
#[remain::sorted]
pub enum AuthenticationProviderType {
  ActiveDirectory,
  Federation,
  Import,
  LDAP,
  Okta,
  Social,
}

crate::impl_as_ref!(AuthenticationProviderType);

#[cfg(feature = "validate")]
impl ::validator::Validate for AuthenticationProviderType {
  fn validate(&self) -> Result<(), ::validator::ValidationErrors> {
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  crate::testing::roundtrip!(super::AuthenticationProviderType);
  crate::testing::validate!(super::AuthenticationProviderType);
}
