#[crate::apply(crate::structs_with_args!)]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct PathGroupId {
  #[cfg_attr(feature = "clap", arg(long, help = "The ID of the group"))]
  pub group_id: String,
}

#[cfg(test)]
mod tests {
  crate::testing::roundtrip!(super::PathGroupId);
  crate::testing::validate!(super::PathGroupId);
}
