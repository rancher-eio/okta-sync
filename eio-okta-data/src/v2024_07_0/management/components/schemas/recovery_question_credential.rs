#[crate::apply(crate::structs!)]
pub struct RecoveryQuestionCredential {
  #[cfg_attr(feature = "proptest", proptest(strategy = "crate::strategy::std::option_string_with_length(1..=100)"))]
  #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none"))]
  #[cfg_attr(feature = "validate", validate(length(min = 1, max = 100)))]
  pub answer: Option<String>,
  #[cfg_attr(feature = "proptest", proptest(strategy = "crate::strategy::std::string_with_length(1..=100)"))]
  #[cfg_attr(feature = "validate", validate(length(min = 1, max = 100)))]
  pub question: String,
}

crate::impl_as_ref!(RecoveryQuestionCredential);

#[cfg(test)]
mod tests {
  crate::testing::roundtrip!(super::RecoveryQuestionCredential);
  crate::testing::validate!(super::RecoveryQuestionCredential);
}
