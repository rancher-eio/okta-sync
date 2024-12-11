#[crate::apply(crate::enums!)]
#[derive(Copy)]
#[cfg_attr(feature = "serde", serde(rename_all = "SCREAMING_SNAKE_CASE"))]
#[remain::sorted]
pub enum GroupType {
  AppGroup,
  BuiltIn,
  OktaGroup,
}

crate::impl_as_ref!(GroupType);

#[cfg(feature = "validate")]
impl ::validator::Validate for GroupType {
  fn validate(&self) -> Result<(), ::validator::ValidationErrors> {
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  crate::testing::roundtrip!(super::GroupType);
  crate::testing::validate!(super::GroupType);
}
