#[crate::apply(crate::structs!)]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct UserProfile {
  #[cfg_attr(feature = "dummy", dummy(faker = "crate::fake::address::custom::CityName()"))]
  #[cfg_attr(feature = "proptest", proptest(strategy = "crate::strategy::std::option_string_with_length(0..=128)"))]
  #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none"))]
  #[cfg_attr(feature = "validate", validate(length(max = 128)))]
  pub city: Option<String>,
  #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none"))]
  pub cost_center: Option<String>,
  #[cfg_attr(feature = "dummy", dummy(faker = "crate::fake::address::custom::CountryCode()"))]
  #[cfg_attr(feature = "proptest", proptest(strategy = "crate::strategy::std::option_string_with_length(0..=2)"))]
  #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none"))]
  #[cfg_attr(feature = "validate", validate(length(max = 2)))]
  pub country_code: Option<String>,
  // deviant
  #[cfg_attr(feature = "dummy", dummy(faker = "crate::fake::job::custom::Field()"))]
  pub department: Option<String>,
  #[cfg_attr(feature = "dummy", dummy(faker = "crate::fake::internet::custom::Username()"))]
  #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none"))]
  pub display_name: Option<String>,
  #[cfg_attr(feature = "dummy", dummy(faker = "crate::fake::job::custom::Field()"))]
  #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none"))]
  pub division: Option<String>,
  #[cfg_attr(feature = "dummy", dummy(faker = "crate::fake::internet::en::SafeEmail()"))]
  #[cfg_attr(feature = "proptest", proptest(strategy("crate::strategy::email::length::within(5..=100)")))]
  #[cfg_attr(feature = "validate", validate(email, length(min = 5, max = 100)))]
  pub email: String,
  // deviant
  pub employee_number: Option<String>,
  #[cfg_attr(feature = "dummy", dummy(faker = "crate::fake::name::custom::FirstName()"))]
  #[cfg_attr(feature = "proptest", proptest(strategy("crate::strategy::std::option_string_with_length(1..=50)")))]
  #[cfg_attr(feature = "validate", validate(length(min = 1, max = 50)))]
  pub first_name: Option<String>,
  // deviant
  #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none"))]
  pub github_orgs: Option<Vec<String>>,
  // deviant
  #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none"))]
  pub github_username: Option<Vec<String>>,
  #[cfg_attr(feature = "dummy", dummy(faker = "crate::fake::name::custom::Title()"))]
  #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none"))]
  pub honorific_prefix: Option<String>,
  #[cfg_attr(feature = "dummy", dummy(faker = "crate::fake::name::custom::Suffix()"))]
  #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none"))]
  pub honorific_suffix: Option<String>,
  #[cfg_attr(feature = "dummy", dummy(faker = "crate::fake::name::custom::LastName()"))]
  #[cfg_attr(feature = "proptest", proptest(strategy = "crate::strategy::std::option_string_with_length(1..=50)"))]
  #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none"))]
  #[cfg_attr(feature = "validate", validate(length(min = 1, max = 50)))]
  pub last_name: Option<String>,
  // deviant
  pub locale: Option<String>,
  #[cfg_attr(feature = "dummy", dummy(faker = "crate::fake::internet::en::Username()"))]
  #[cfg_attr(feature = "proptest", proptest(strategy = "crate::strategy::std::string_with_length(5..=100)"))]
  #[cfg_attr(feature = "validate", validate(length(min = 5, max = 100)))]
  pub login: String,
  #[cfg_attr(feature = "dummy", dummy(faker = "crate::fake::name::custom::Name()"))]
  #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none"))]
  pub manager: Option<String>,
  #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none"))]
  pub manager_id: Option<String>,
  #[cfg_attr(feature = "dummy", dummy(faker = "crate::fake::name::custom::FirstName()"))]
  #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none"))]
  pub middle_name: Option<String>,
  #[cfg_attr(feature = "dummy", dummy(faker = "crate::fake::phone_number::custom::CellNumber()"))]
  #[cfg_attr(feature = "proptest", proptest(strategy = "crate::strategy::std::option_string_with_length(0..=100)"))]
  #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none"))]
  #[cfg_attr(feature = "validate", validate(length(min = 0, max = 100)))]
  pub mobile_phone: Option<String>,
  #[cfg_attr(feature = "dummy", dummy(faker = "crate::fake::name::custom::FirstName()"))]
  #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none"))]
  pub nick_name: Option<String>,
  #[cfg_attr(feature = "dummy", dummy(faker = "crate::fake::company::custom::CompanyName()"))]
  #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none"))]
  pub organization: Option<String>,
  #[cfg_attr(feature = "dummy", dummy(faker = "crate::fake::address::custom::PostalAddress()"))]
  #[cfg_attr(feature = "proptest", proptest(strategy = "crate::strategy::std::option_string_with_length(0..=4096)"))]
  #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none"))]
  #[cfg_attr(feature = "validate", validate(length(max = 4096)))]
  pub postal_address: Option<String>,
  #[cfg_attr(feature = "dummy", dummy(faker = "crate::fake::locale::custom::Language()"))]
  #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none"))]
  pub preferred_language: Option<String>,
  #[cfg_attr(feature = "dummy", dummy(faker = "crate::fake::phone_number::custom::PhoneNumber()"))]
  #[cfg_attr(feature = "proptest", proptest(strategy = "crate::strategy::std::option_string_with_length(0..=100)"))]
  #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none"))]
  #[cfg_attr(feature = "validate", validate(length(min = 0, max = 100)))]
  pub primary_phone: Option<String>,
  #[cfg_attr(feature = "dummy", dummy(expr = "None"))]
  #[cfg_attr(feature = "proptest", proptest(strategy("crate::strategy::url::option()")))]
  #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none"))]
  #[cfg_attr(feature = "validate", validate(url))]
  pub profile_url: Option<String>,
  #[cfg_attr(feature = "dummy", dummy(faker = "crate::fake::internet::custom::FreeEmail()"))]
  #[cfg_attr(feature = "proptest", proptest(strategy = "crate::strategy::email::length::within::option(5..=100)"))]
  #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none"))]
  #[cfg_attr(feature = "validate", validate(email, length(min = 5, max = 100)))]
  pub second_email: Option<String>,
  #[cfg_attr(feature = "dummy", dummy(faker = "crate::fake::address::custom::StateName()"))]
  #[cfg_attr(feature = "proptest", proptest(strategy = "crate::strategy::std::option_string_with_length(0..=128)"))]
  #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none"))]
  #[cfg_attr(feature = "validate", validate(length(max = 128)))]
  pub state: Option<String>,
  #[cfg_attr(feature = "dummy", dummy(faker = "crate::fake::address::custom::StreetAddress()"))]
  #[cfg_attr(feature = "proptest", proptest(strategy = "crate::strategy::std::option_string_with_length(0..=1024)"))]
  #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none"))]
  #[cfg_attr(feature = "validate", validate(length(max = 1024)))]
  pub street_address: Option<String>,
  #[cfg_attr(feature = "dummy", dummy(faker = "crate::fake::address::custom::TimeZone()"))]
  #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none"))]
  pub timezone: Option<String>,
  #[cfg_attr(feature = "dummy", dummy(faker = "crate::fake::job::custom::Title()"))]
  #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none"))]
  pub title: Option<String>,
  #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none"))]
  pub user_type: Option<String>,
  #[cfg_attr(feature = "dummy", dummy(faker = "crate::fake::address::custom::ZipCode()"))]
  #[cfg_attr(feature = "proptest", proptest(strategy = "crate::strategy::std::option_string_with_length(0..=50)"))]
  #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none"))]
  #[cfg_attr(feature = "validate", validate(length(max = 50)))]
  pub zip_code: Option<String>,
}

crate::impl_as_ref!(UserProfile);

#[cfg(test)]
mod tests {
  crate::testing::roundtrip!(super::UserProfile);
  crate::testing::validate!(super::UserProfile);
}
