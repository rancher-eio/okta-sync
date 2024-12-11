#[crate::apply(crate::structs_with_args!)]
#[derive(Copy)]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
pub struct QueryUserExpand {
  #[cfg_attr(feature = "clap", arg(long, help = "An optional parameter to include metadata in the `_embedded` attribute"))]
  pub expand: Option<UserExpand>,
}

#[crate::apply(crate::enums!)]
#[derive(Copy)]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
pub enum UserExpand {
  Blocks,
}

#[cfg(test)]
mod tests {
  crate::testing::roundtrip!(super::QueryUserExpand);
  crate::testing::validate!(super::QueryUserExpand);
}
