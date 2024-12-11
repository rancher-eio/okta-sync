#[crate::apply(crate::structs!)]
pub struct PasswordCredentialHash {
  pub algorithm: super::PasswordCredentialHashAlgorithm,
  pub digest: super::DigestAlgorithm,
  pub iteration_count: u64,
  pub key_size: u64,
  pub salt: String,
  pub value: String,
  #[cfg_attr(feature = "dummy", dummy(faker = "1..=20"))]
  #[cfg_attr(feature = "proptest", proptest(strategy = "1..=20u8"))]
  #[cfg_attr(feature = "validate", validate(range(min = 1, max = 20)))]
  pub work_factor: u8,
}

crate::impl_as_ref!(PasswordCredentialHash);
crate::impl_as_ref!(PasswordCredentialHash => super::PasswordCredentialHashAlgorithm as algorithm);
crate::impl_as_ref!(PasswordCredentialHash => super::DigestAlgorithm as digest);
crate::impl_from!(PasswordCredentialHash => super::PasswordCredentialHashAlgorithm as algorithm);
crate::impl_from!(PasswordCredentialHash => super::DigestAlgorithm as digest);

#[cfg(test)]
mod tests {
  crate::testing::roundtrip!(super::PasswordCredentialHash);
  crate::testing::validate!(super::PasswordCredentialHash);
}
