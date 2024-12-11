#[crate::apply(crate::structs!)]
pub struct PasswordCredentialHook {
  #[cfg_attr(feature = "serde", serde(rename = "type"))]
  pub type_: String,
}

crate::impl_as_ref!(PasswordCredentialHook);

#[cfg(test)]
mod tests {
  crate::testing::roundtrip!(super::PasswordCredentialHook);
  crate::testing::validate!(super::PasswordCredentialHook);
}
