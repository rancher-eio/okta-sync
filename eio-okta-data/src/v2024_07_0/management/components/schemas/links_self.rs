#[crate::apply(crate::structs!)]
pub struct LinksSelf {
  #[cfg_attr(feature = "serde", serde(rename = "self"))]
  self_link: super::HrefObjectSelfLink,
}

crate::impl_as_ref!(LinksSelf);
crate::impl_as_ref!(LinksSelf => super::HrefObjectSelfLink as self_link);
crate::impl_from!(LinksSelf => super::HrefObjectSelfLink as self_link);

#[cfg(test)]
mod tests {
  crate::testing::roundtrip!(super::LinksSelf);
  crate::testing::validate!(super::LinksSelf);
}
