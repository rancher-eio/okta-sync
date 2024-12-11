use chrono::{DateTime, Utc};
use serde_json::Value;

#[crate::apply(crate::structs!)]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct Group {
  #[cfg_attr(feature = "comparable", comparable_ignore)]
  #[cfg_attr(feature = "comparable", comparable_synthetic { let created = |this: &Self| -> crate::comparable::DateTime { this.created.into() };})]
  #[cfg_attr(all(feature = "patch", feature = "comparable"), patch(attribute(comparable_ignore)))]
  #[cfg_attr(all(feature = "patch", feature = "comparable"), patch(attribute(comparable_synthetic { let created = |this: &Self| -> Option<crate::comparable::DateTime> { this.created.map(Into::into) };})))]
  #[cfg_attr(feature = "proptest", proptest(strategy = "crate::strategy::chrono::datetime()"))]
  pub created: DateTime<Utc>,
  // deviant
  #[cfg_attr(feature = "arbitrary", arbitrary(value = None))]
  #[cfg_attr(feature = "dummy", dummy(expr = "None"))]
  #[cfg_attr(feature = "comparable", comparable_ignore)]
  #[cfg_attr(all(feature = "patch", feature = "comparable"), patch(attribute(comparable_ignore)))]
  #[cfg_attr(feature = "proptest", proptest(value = "None"))]
  #[cfg_attr(feature = "serde", serde(rename = "_embedded"))]
  pub embedded: Option<Value>,
  pub id: String,
  #[cfg_attr(feature = "comparable", comparable_ignore)]
  #[cfg_attr(feature = "comparable", comparable_synthetic { let last_membership_updated = |this: &Self| -> crate::comparable::DateTime { this.last_membership_updated.into() };})]
  #[cfg_attr(all(feature = "patch", feature = "comparable"), patch(attribute(comparable_ignore)))]
  #[cfg_attr(all(feature = "patch", feature = "comparable"), patch(attribute(comparable_synthetic { let last_membership_updated = |this: &Self| -> Option<crate::comparable::DateTime> { this.last_membership_updated.map(Into::into) };})))]
  #[cfg_attr(feature = "proptest", proptest(strategy = "crate::strategy::chrono::datetime()"))]
  pub last_membership_updated: DateTime<Utc>,
  #[cfg_attr(feature = "comparable", comparable_ignore)]
  #[cfg_attr(feature = "comparable", comparable_synthetic { let last_updated = |this: &Self| -> crate::comparable::DateTime { this.last_updated.into() };})]
  #[cfg_attr(all(feature = "patch", feature = "comparable"), patch(attribute(comparable_ignore)))]
  #[cfg_attr(all(feature = "patch", feature = "comparable"), patch(attribute(comparable_synthetic { let last_updated = |this: &Self| -> Option<crate::comparable::DateTime> { this.last_updated.map(Into::into) };})))]
  #[cfg_attr(feature = "proptest", proptest(strategy = "crate::strategy::chrono::datetime()"))]
  pub last_updated: DateTime<Utc>,
  #[cfg_attr(feature = "comparable", comparable_ignore)]
  #[cfg_attr(feature = "serde", serde(rename = "_links"))]
  pub links: Links,
  pub object_class: Vec<String>,
  pub profile: super::GroupProfile,
  #[cfg_attr(feature = "serde", serde(rename = "type"))]
  pub type_: super::GroupType,
}

crate::impl_as_ref!(Group => Links as links);
crate::impl_as_ref!(Group => super::GroupProfile as profile);
crate::impl_as_ref!(Group => super::GroupType as type_);
crate::impl_as_ref!(Group);
crate::impl_from!(Group => Links as links);
crate::impl_from!(Group => super::GroupProfile as profile);
crate::impl_from!(Group => super::GroupType as type_);

#[crate::apply(crate::structs!)]
pub struct Links {
  pub apps: super::HrefObject,
  pub logo: Vec<super::HrefObject>,
  #[cfg_attr(feature = "serde", serde(flatten))]
  // deviant
  pub self_link: Option<super::LinksSelf>,
  // deviant
  pub source: Option<super::HrefObject>,
  pub users: super::HrefObject,
}

crate::impl_as_ref!(Links);

// #[cfg(feature = "arbitrary")]
// impl<'a> ::arbitrary::Arbitrary<'a> for Group {
//   fn arbitrary(u: &mut ::arbitrary::Unstructured<'a>) -> ::arbitrary::Result<Self> {
//     Ok(Self {
//       created: DateTime::<Utc>::arbitrary(u)?,
//       embedded: None,
//       id: String::arbitrary(u)?,
//       last_membership_updated: DateTime::<Utc>::arbitrary(u)?,
//       last_updated: DateTime::<Utc>::arbitrary(u)?,
//       links: Links::arbitrary(u)?,
//       object_class: Vec::<String>::arbitrary(u)?,
//       profile: super::GroupProfile::arbitrary(u)?,
//       type_: super::GroupType::arbitrary(u)?,
//     })
//   }
// }

// #[cfg(feature = "dummy")]
// impl ::fake::Dummy<::fake::Faker> for Group {
//   fn dummy_with_rng<R: ::proptest::prelude::Rng + ?Sized>(config: &::fake::Faker, rng: &mut R) -> Self {
//     Self {
//       created: DateTime::<Utc>::dummy_with_rng(config, rng),
//       embedded: None,
//       id: String::dummy_with_rng(config, rng),
//       last_membership_updated: DateTime::<Utc>::dummy_with_rng(config, rng),
//       last_updated: DateTime::<Utc>::dummy_with_rng(config, rng),
//       links: Links::dummy_with_rng(config, rng),
//       object_class: Vec::<String>::dummy_with_rng(config, rng),
//       profile: super::GroupProfile::dummy_with_rng(config, rng),
//       type_: super::GroupType::dummy_with_rng(config, rng),
//     }
//   }
// }

#[cfg(test)]
mod tests {
  crate::testing::roundtrip!(super::Group);
  crate::testing::validate!(super::Group);
}
