#[crate::apply(crate::structs!)]
pub struct AuthenticationProvider {
  pub name: String,
  #[cfg_attr(feature = "serde", serde(rename = "type"))]
  pub type_: super::AuthenticationProviderType,
}

crate::impl_as_ref!(AuthenticationProvider);
crate::impl_as_ref!(AuthenticationProvider => super::AuthenticationProviderType as type_);
crate::impl_from!(AuthenticationProvider => super::AuthenticationProviderType as type_);

#[cfg(test)]
mod tests {
  crate::testing::roundtrip!(super::AuthenticationProvider);
  crate::testing::validate!(super::AuthenticationProvider);
}
