#[crate::apply(crate::structs!)]
pub struct GroupProfile {
  // deviant
  pub description: Option<String>,
  #[cfg_attr(feature = "dummy", dummy(faker = "::fake::faker::company::en::Profession()"))]
  pub name: String,
}

crate::impl_as_ref!(GroupProfile);

#[cfg(test)]
mod tests {
  crate::testing::roundtrip!(super::GroupProfile);
  crate::testing::validate!(super::GroupProfile);
}
