use chrono::{DateTime, Utc};
use serde_json::Value;

#[crate::apply(crate::structs!)]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct User {
  #[cfg_attr(feature = "comparable", comparable_ignore)]
  #[cfg_attr(feature = "comparable", comparable_synthetic { let activated = |this: &Self| -> Option<crate::comparable::DateTime> { this.activated.map(Into::into) };})]
  #[cfg_attr(all(feature = "patch", feature = "comparable"), patch(attribute(comparable_ignore)))]
  #[cfg_attr(all(feature = "patch", feature = "comparable"), patch(attribute(comparable_synthetic { let activated = |this: &Self| -> Option<Option<crate::comparable::DateTime>> { this.activated.map(|activated| activated.map(Into::into)) };})))]
  #[cfg_attr(feature = "proptest", proptest(strategy = "crate::strategy::chrono::option_datetime()"))]
  #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none"))]
  pub activated: Option<DateTime<Utc>>,
  #[cfg_attr(feature = "comparable", comparable_ignore)]
  #[cfg_attr(feature = "comparable", comparable_synthetic { let created = |this: &Self| -> crate::comparable::DateTime { this.created.into() };})]
  #[cfg_attr(all(feature = "patch", feature = "comparable"), patch(attribute(comparable_ignore)))]
  #[cfg_attr(all(feature = "patch", feature = "comparable"), patch(attribute(comparable_synthetic { let created = |this: &Self| -> Option<crate::comparable::DateTime> { this.created.map(Into::into) };})))]
  #[cfg_attr(feature = "proptest", proptest(strategy = "crate::strategy::chrono::datetime()"))]
  pub created: DateTime<Utc>,
  pub credentials: super::UserCredentials,
  #[cfg_attr(feature = "arbitrary", arbitrary(value = None))]
  #[cfg_attr(feature = "dummy", dummy(expr = "None"))]
  #[cfg_attr(feature = "comparable", comparable_ignore)]
  #[cfg_attr(all(feature = "patch", feature = "comparable"), patch(attribute(comparable_ignore)))]
  #[cfg_attr(feature = "proptest", proptest(value = "None"))]
  #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none"))]
  #[cfg_attr(feature = "serde", serde(rename = "_embedded"))]
  pub embedded: Option<Value>,
  pub id: String,
  #[cfg_attr(feature = "comparable", comparable_ignore)]
  #[cfg_attr(feature = "comparable", comparable_synthetic { let last_login = |this: &Self| -> Option<crate::comparable::DateTime> { this.last_login.map(Into::into) };})]
  #[cfg_attr(all(feature = "patch", feature = "comparable"), patch(attribute(comparable_ignore)))]
  #[cfg_attr(all(feature = "patch", feature = "comparable"), patch(attribute(comparable_synthetic { let last_login = |this: &Self| -> Option<Option<crate::comparable::DateTime>> { this.last_login.map(|activated| activated.map(Into::into)) };})))]
  #[cfg_attr(feature = "proptest", proptest(strategy = "crate::strategy::chrono::option_datetime()"))]
  #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none"))]
  pub last_login: Option<DateTime<Utc>>,
  #[cfg_attr(feature = "comparable", comparable_ignore)]
  #[cfg_attr(feature = "comparable", comparable_synthetic { let last_updated = |this: &Self| -> Option<crate::comparable::DateTime> { this.last_updated.map(Into::into) };})]
  #[cfg_attr(all(feature = "patch", feature = "comparable"), patch(attribute(comparable_ignore)))]
  #[cfg_attr(all(feature = "patch", feature = "comparable"), patch(attribute(comparable_synthetic { let last_updated = |this: &Self| -> Option<Option<crate::comparable::DateTime>> { this.last_updated.map(|activated| activated.map(Into::into)) };})))]
  #[cfg_attr(feature = "proptest", proptest(strategy = "crate::strategy::chrono::option_datetime()"))]
  #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none"))]
  pub last_updated: Option<DateTime<Utc>>,
  #[cfg_attr(feature = "comparable", comparable_ignore)]
  #[cfg_attr(feature = "serde", serde(rename = "_links"))]
  pub links: Links,
  #[cfg_attr(feature = "comparable", comparable_ignore)]
  #[cfg_attr(feature = "comparable", comparable_synthetic { let password_changed = |this: &Self| -> Option<crate::comparable::DateTime> { this.password_changed.map(Into::into) };})]
  #[cfg_attr(all(feature = "patch", feature = "comparable"), patch(attribute(comparable_ignore)))]
  #[cfg_attr(all(feature = "patch", feature = "comparable"), patch(attribute(comparable_synthetic { let password_changed = |this: &Self| -> Option<Option<crate::comparable::DateTime>> { this.password_changed.map(|activated| activated.map(Into::into)) };})))]
  #[cfg_attr(feature = "proptest", proptest(strategy = "crate::strategy::chrono::option_datetime()"))]
  #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none"))]
  pub password_changed: Option<DateTime<Utc>>,
  pub profile: super::UserProfile,
  #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none"))]
  pub realm_id: Option<String>,
  pub status: super::UserStatus,
  #[cfg_attr(feature = "comparable", comparable_ignore)]
  #[cfg_attr(feature = "comparable", comparable_synthetic { let status_changed = |this: &Self| -> Option<crate::comparable::DateTime> { this.status_changed.map(Into::into) };})]
  #[cfg_attr(all(feature = "patch", feature = "comparable"), patch(attribute(comparable_ignore)))]
  #[cfg_attr(all(feature = "patch", feature = "comparable"), patch(attribute(comparable_synthetic { let status_changed = |this: &Self| -> Option<Option<crate::comparable::DateTime>> { this.status_changed.map(|activated| activated.map(Into::into)) };})))]
  #[cfg_attr(feature = "proptest", proptest(strategy = "crate::strategy::chrono::option_datetime()"))]
  #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none"))]
  pub status_changed: Option<DateTime<Utc>>,
  #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none"))]
  pub transitioning_to_status: Option<super::UserStatusTransitioningTo>,
  #[cfg_attr(feature = "serde", serde(rename = "type"))]
  pub type_: super::UserType,
}

crate::impl_as_ref!(User => Links as links);
crate::impl_as_ref!(User => super::UserCredentials as credentials);
crate::impl_as_ref!(User => super::UserProfile as profile);
crate::impl_as_ref!(User => super::UserStatus as status);
crate::impl_as_ref!(User);
crate::impl_from!(User => Links as links);
crate::impl_from!(User => super::UserCredentials as credentials);
crate::impl_from!(User => super::UserProfile as profile);
crate::impl_from!(User => super::UserStatus as status);

#[crate::apply(crate::structs!)]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct Links {
  // deviant
  pub activate: Option<super::HrefObject>,
  // deviant
  pub change_password: Option<super::HrefObject>,
  // deviant
  pub change_recovery_question: Option<super::HrefObject>,
  // deviant
  pub deactivate: Option<super::HrefObject>,
  // deviant
  pub expire_password: Option<super::HrefObject>,
  // deviant
  pub forgot_password: Option<super::HrefObject>,
  // deviant
  pub reactivate: Option<super::HrefObject>,
  // deviant
  pub reset_factors: Option<super::HrefObject>,
  // deviant
  pub reset_password: Option<super::HrefObject>,
  // deviant
  pub schema: Option<super::HrefObject>,
  #[cfg_attr(feature = "serde", serde(flatten))]
  pub self_link: super::LinksSelf,
  // deviant
  pub suspend: Option<super::HrefObject>,
  #[cfg_attr(feature = "serde", serde(rename = "type"))]
  // deviant
  pub type_: Option<super::HrefObject>,
  // deviant
  pub unlock: Option<super::HrefObject>,
  // deviant
  pub unsuspend: Option<super::HrefObject>,
}

crate::impl_as_ref!(Links);
crate::impl_as_ref!(Links => super::LinksSelf as self_link);
crate::impl_from!(Links => super::LinksSelf as self_link);

// #[cfg(feature = "arbitrary")]
// impl<'a> ::arbitrary::Arbitrary<'a> for User {
//   fn arbitrary(u: &mut ::arbitrary::Unstructured<'a>) -> ::arbitrary::Result<Self> {
//     Ok(Self {
//       activated: Option::<DateTime<Utc>>::arbitrary(u)?,
//       created: DateTime::<Utc>::arbitrary(u)?,
//       credentials: super::UserCredentials::arbitrary(u)?,
//       embedded: None,
//       id: String::arbitrary(u)?,
//       last_login: Option::<DateTime<Utc>>::arbitrary(u)?,
//       last_updated: Option::<DateTime<Utc>>::arbitrary(u)?,
//       links: Links::arbitrary(u)?,
//       password_changed: Option::<DateTime<Utc>>::arbitrary(u)?,
//       profile: super::UserProfile::arbitrary(u)?,
//       realm_id: Option::<String>::arbitrary(u)?,
//       status: super::UserStatus::arbitrary(u)?,
//       status_changed: Option::<DateTime<Utc>>::arbitrary(u)?,
//       transitioning_to_status: Option::<super::UserStatusTransitioningTo>::arbitrary(u)?,
//       type_: super::UserType::arbitrary(u)?,
//     })
//   }
// }

// #[cfg(feature = "dummy")]
// impl ::fake::Dummy<::fake::Faker> for User {
//   fn dummy_with_rng<R: ::proptest::prelude::Rng + ?Sized>(config: &::fake::Faker, rng: &mut R) -> Self {
//     Self {
//       activated: Option::<DateTime<Utc>>::dummy_with_rng(config, rng),
//       created: DateTime::<Utc>::dummy_with_rng(config, rng),
//       credentials: super::UserCredentials::dummy_with_rng(config, rng),
//       embedded: None,
//       id: String::dummy_with_rng(config, rng),
//       last_login: Option::<DateTime<Utc>>::dummy_with_rng(config, rng),
//       last_updated: Option::<DateTime<Utc>>::dummy_with_rng(config, rng),
//       links: Links::dummy_with_rng(config, rng),
//       password_changed: Option::<DateTime<Utc>>::dummy_with_rng(config, rng),
//       profile: super::UserProfile::dummy_with_rng(config, rng),
//       realm_id: Option::<String>::dummy_with_rng(config, rng),
//       status: super::UserStatus::dummy_with_rng(config, rng),
//       status_changed: Option::<DateTime<Utc>>::dummy_with_rng(config, rng),
//       transitioning_to_status: Option::<super::UserStatusTransitioningTo>::dummy_with_rng(config, rng),
//       type_: super::UserType::dummy_with_rng(config, rng),
//     }
//   }
// }

#[cfg(test)]
mod tests {
  crate::testing::roundtrip!(super::User);
  crate::testing::validate!(super::User);
}
