use mediatype::MediaTypeBuf;

#[crate::apply(crate::structs!)]
pub struct HrefObject {
  // deviant
  pub hints: Option<super::HrefHints>,
  pub href: String,
  // deviant
  pub name: Option<String>,
  // deviant
  pub templated: Option<bool>,
  #[cfg_attr(feature = "arbitrary", arbitrary(value = ::mediatype::media_type!(APPLICATION / JSON).into()))]
  #[cfg_attr(feature = "dummy", dummy(expr = "::mediatype::media_type!(APPLICATION / JSON).into()"))]
  #[cfg_attr(feature = "comparable", comparable_ignore)]
  #[cfg_attr(feature = "comparable", comparable_synthetic { let type_ = |this: &Self| -> crate::comparable::MediaType { this.type_.to_ref().into() };})]
  #[cfg_attr(all(feature = "patch", feature = "comparable"), patch(attribute(comparable_ignore)))]
  #[cfg_attr(all(feature = "patch", feature = "comparable"), patch(attribute(comparable_synthetic { let type_ = |this: &Self| -> Option<crate::comparable::MediaType> { this.type_.as_ref().map(|t| t.to_ref().into()) };})))]
  #[cfg_attr(feature = "proptest", proptest(value = "application_json()"))]
  #[cfg_attr(feature = "schemars", schemars(skip))]
  #[cfg_attr(all(feature = "schemars", feature = "patch"), patch(attribute(schemars(skip))))]
  #[cfg_attr(feature = "serde", serde(rename = "type", default = "application_json"))]
  pub type_: MediaTypeBuf,
}

crate::impl_as_ref!(HrefObject);
crate::impl_from!(HrefObject => Option<super::HrefHints> as hints);

#[cfg(any(feature = "serde", feature = "proptest", feature = "arbitrary", feature = "dummy"))]
fn application_json() -> MediaTypeBuf {
  ::mediatype::media_type!(APPLICATION / JSON).into()
}

// #[cfg(feature = "arbitrary")]
// impl<'a> ::arbitrary::Arbitrary<'a> for HrefObject {
//   fn arbitrary(u: &mut ::arbitrary::Unstructured<'a>) -> ::arbitrary::Result<Self> {
//     Ok(Self {
//       hints: Option::<super::HrefHints>::arbitrary(u)?,
//       href: String::arbitrary(u)?,
//       name: Option::<String>::arbitrary(u)?,
//       templated: Option::<bool>::arbitrary(u)?,
//       type_: application_json(),
//     })
//   }
// }

// #[cfg(feature = "dummy")]
// impl ::fake::Dummy<::fake::Faker> for HrefObject {
//   fn dummy_with_rng<R: ::proptest::prelude::Rng + ?Sized>(config: &::fake::Faker, rng: &mut R) -> Self {
//     Self {
//       hints: Option::<super::HrefHints>::dummy_with_rng(config, rng),
//       href: String::dummy_with_rng(config, rng),
//       name: Option::<String>::dummy_with_rng(config, rng),
//       templated: Option::<bool>::dummy_with_rng(config, rng),
//       type_: application_json(),
//     }
//   }
// }

#[cfg(test)]
mod tests {
  crate::testing::roundtrip!(super::HrefObject);
  crate::testing::validate!(super::HrefObject);
}
