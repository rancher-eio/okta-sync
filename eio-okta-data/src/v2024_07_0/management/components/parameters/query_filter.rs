#[crate::apply(crate::structs_with_args!)]
#[derive(Default)]
pub struct QueryFilter {
  #[cfg_attr(feature = "clap", arg(long, help = "Searches the records for matching value",))]
  #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none"))]
  pub after: Option<String>,
}

#[cfg(test)]
mod tests {
  crate::testing::roundtrip!(super::QueryFilter);
  crate::testing::validate!(super::QueryFilter);
}
