#[crate::apply(crate::structs!)]
#[derive(Default)]
pub struct HrefHints {
  pub allow: Vec<super::HttpMethod>,
}

crate::impl_as_ref!(HrefHints);

impl AsRef<[super::HttpMethod]> for HrefHints {
  fn as_ref(&self) -> &[super::HttpMethod] {
    self.allow.as_slice()
  }
}

#[cfg(test)]
mod tests {
  crate::testing::roundtrip!(super::HrefHints);
  crate::testing::validate!(super::HrefHints);
}
