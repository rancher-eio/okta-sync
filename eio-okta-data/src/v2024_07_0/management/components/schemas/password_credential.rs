#[crate::apply(crate::structs!)]
pub struct PasswordCredential {
  // deviant
  pub hash: Option<super::PasswordCredentialHash>,
  // deviant
  pub hook: Option<super::PasswordCredentialHook>,
  #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none"))]
  pub value: Option<String>,
}

crate::impl_as_ref!(PasswordCredential);

#[cfg(test)]
mod tests {
  crate::testing::roundtrip!(super::PasswordCredential);
  crate::testing::validate!(super::PasswordCredential);
}
