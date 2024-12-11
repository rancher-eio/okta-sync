#[crate::apply(crate::structs_with_args!)]
#[derive(Default)]
pub struct QueryAfter {
  #[cfg_attr(feature = "clap", arg(long, help = "The cursor to use for pagination",))]
  #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none"))]
  pub after: Option<String>,
}

#[cfg(test)]
mod tests {
  crate::testing::roundtrip!(super::QueryAfter);
  crate::testing::validate!(super::QueryAfter);
}
