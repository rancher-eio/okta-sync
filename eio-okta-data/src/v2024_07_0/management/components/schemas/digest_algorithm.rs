#[allow(non_camel_case_types)]
#[crate::apply(crate::enums!)]
#[derive(Copy)]
#[cfg_attr(feature = "serde", serde(rename_all = "SCREAMING_SNAKE_CASE"))]
#[remain::sorted]
pub enum DigestAlgorithm {
  SHA256_HMAC,
  SHA512_HMAC,
}

crate::impl_as_ref!(DigestAlgorithm);

#[cfg(feature = "validate")]
impl ::validator::Validate for DigestAlgorithm {
  fn validate(&self) -> Result<(), ::validator::ValidationErrors> {
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  crate::testing::roundtrip!(super::DigestAlgorithm);
  crate::testing::validate!(super::DigestAlgorithm);
}
