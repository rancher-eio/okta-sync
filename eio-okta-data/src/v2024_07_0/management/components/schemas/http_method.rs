#[crate::apply(crate::enums!)]
#[derive(Copy)]
#[cfg_attr(feature = "serde", serde(rename_all = "UPPERCASE"))]
#[remain::sorted]
pub enum HttpMethod {
  DELETE,
  GET,
  POST,
  PUT,
}

crate::impl_as_ref!(HttpMethod);

#[cfg(feature = "validate")]
impl ::validator::Validate for HttpMethod {
  fn validate(&self) -> Result<(), ::validator::ValidationErrors> {
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  crate::testing::roundtrip!(super::HttpMethod);
  crate::testing::validate!(super::HttpMethod);
}
