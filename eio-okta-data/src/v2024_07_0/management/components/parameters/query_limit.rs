#[crate::apply(crate::structs_with_args!)]
#[derive(Copy)]
pub struct QueryLimit {
  #[cfg_attr(feature = "clap", arg(long, default_value = "20", help = "A limit on the number of objects to return"))]
  #[cfg_attr(feature = "dummy", dummy(faker = "1..=200"))]
  #[cfg_attr(feature = "proptest", proptest(strategy = "1..=200"))]
  #[cfg_attr(feature = "validate", validate(range(min = 1, max = 200)))]
  pub limit: i32,
}

impl Default for QueryLimit {
  fn default() -> Self {
    Self { limit: 20 }
  }
}

#[cfg(test)]
mod tests {
  crate::testing::roundtrip!(super::QueryLimit);
  crate::testing::validate!(super::QueryLimit);
  crate::testing::defaults_match!(super::QueryLimit);
}
