#[crate::apply(crate::enums!)]
#[derive(Copy)]
#[cfg_attr(feature = "serde", serde(rename_all = "SCREAMING_SNAKE_CASE"))]
#[remain::sorted]
pub enum UserStatus {
  Active,
  Deprovisioned,
  LockedOut,
  PasswordExpired,
  Provisioned,
  Staged,
  Suspended,
}

crate::impl_as_ref!(UserStatus);

#[cfg(feature = "validate")]
impl ::validator::Validate for UserStatus {
  fn validate(&self) -> Result<(), ::validator::ValidationErrors> {
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  crate::testing::roundtrip!(super::UserStatus);
  crate::testing::validate!(super::UserStatus);
}
