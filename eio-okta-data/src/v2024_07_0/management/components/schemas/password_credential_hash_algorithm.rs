#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[crate::apply(crate::enums!)]
#[derive(Copy)]
#[cfg_attr(feature = "serde", serde(rename_all = "SCREAMING-KEBAB-CASE"))]
#[remain::sorted]
pub enum PasswordCredentialHashAlgorithm {
  BCRYPT,
  MD5,
  PBKDF2,
  SHA_1,
  SHA_256,
  SHA_512,
}

crate::impl_as_ref!(PasswordCredentialHashAlgorithm);

#[cfg(feature = "validate")]
impl ::validator::Validate for PasswordCredentialHashAlgorithm {
  fn validate(&self) -> Result<(), ::validator::ValidationErrors> {
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  crate::testing::roundtrip!(super::PasswordCredentialHashAlgorithm);
  crate::testing::validate!(super::PasswordCredentialHashAlgorithm);
}
