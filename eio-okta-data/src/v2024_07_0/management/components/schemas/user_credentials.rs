#[crate::apply(crate::structs!)]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub struct UserCredentials {
  // deviant
  #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none"))]
  pub password: Option<super::PasswordCredential>,
  pub provider: super::AuthenticationProvider,
  // deviant
  pub recovery_question: Option<super::RecoveryQuestionCredential>,
}

crate::impl_as_ref!(UserCredentials);
crate::impl_as_ref!(UserCredentials => super::AuthenticationProvider as provider);
crate::impl_from!(UserCredentials => super::AuthenticationProvider as provider);

#[cfg(test)]
mod tests {
  crate::testing::roundtrip!(super::UserCredentials);
  crate::testing::validate!(super::UserCredentials);
}
