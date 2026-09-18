use authentik_client::models::User as AuthentikUser;
use eio_okta_data::current::management::components::schemas::{
  AuthenticationProvider, AuthenticationProviderType, HrefObject, LinksSelf, User as OktaUser, UserCredentials,
  UserProfile, UserStatus, UserType, user::Links,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::HashMap, sync::LazyLock};

use crate::okta::UserProfileExtensions;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct User(AuthentikUser);

impl From<AuthentikUser> for User {
  fn from(user: AuthentikUser) -> Self {
    Self(user)
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[remain::sorted]
pub(crate) struct Attributes {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub address: Option<Address>,
  #[serde(rename = "cn")]
  pub cn: String,
  #[serde(rename = "communityUid")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub community_uid: Option<String>,
  #[serde(rename = "costCenter")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub cost_center: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub department: Option<String>,
  #[serde(rename = "distinguishedName")]
  pub distinguished_name: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub division: Option<String>,
  #[serde(rename = "employeeNumber")]
  pub employee_number: String,
  #[serde(rename = "employeeStartDate")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub employee_start_date: Option<String>,
  #[serde(rename = "employeeStatus")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub employee_status: Option<String>,
  #[serde(rename = "employeeType")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub employee_type: Option<String>,
  #[serde(default)]
  pub entitlements: Vec<String>,
  #[serde(rename = "givenName")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub given_name: Option<String>,
  #[serde(rename = "isManager")]
  #[serde(skip_serializing_if = "Option::is_none")]
  is_manager: Option<String>,
  #[serde(rename = "isSupervisor")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub is_supervisor: Option<bool>,
  #[serde(rename = "itStack")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub it_stack: Option<String>,
  #[serde(rename = "jobCode")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub job_code: Option<String>,
  #[serde(rename = "jobFamily")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub job_family: Option<String>,
  #[serde(rename = "jobfamilygroup")]
  #[serde(skip_serializing_if = "Option::is_none")]
  job_family_group: Option<String>,
  #[serde(rename = "ldap_uniq")]
  pub ldap_uniq: String,
  #[serde(default)]
  pub mail: Vec<String>,
  #[serde(rename = "managerEmail")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub manager_email: Option<String>,
  #[serde(rename = "managerUid")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub manager_uid: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub mobile: Option<String>,
  #[serde(rename = "modifyTimestamp")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub modify_timestamp: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub office: Option<String>,
  #[serde(rename = "oktaAnonEmail")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub okta_anon_email: Option<String>,
  #[serde(rename = "oktaAnonEmail.bak")]
  #[serde(skip_serializing_if = "Option::is_none")]
  okta_anon_email_bak: Option<String>,
  #[serde(rename = "oktaId")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub okta_id: Option<OktaId>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub organization: Option<String>,
  #[serde(rename = "secondaryWorkforceManagerID")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub secondary_workforce_manager_id: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub sn: Option<String>,
  #[serde(rename = "socialId")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub social_id: Option<SocialId>,
  #[serde(default)]
  #[serde(rename = "sshPublicKey")]
  pub ssh_public_key: Vec<String>,
  #[serde(rename = "telephoneNumber")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub telephone_number: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub title: Option<String>,
  pub uid: String,
  #[serde(rename = "uidNumber")]
  pub uid_number: Option<i64>,
  #[serde(rename = "upn")]
  pub upn: String,
  #[serde(rename = "userType")]
  #[serde(skip_serializing_if = "Option::is_none")]
  user_type: Option<String>,
  #[serde(rename = "userdbAccountsSuseId")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub userdb_accounts_suse_id: Option<i64>,
  #[serde(rename = "userdbLogin")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub userdb_login: Option<String>,
  #[serde(rename = "userdbPeopleSuseId")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub userdb_people_suse_id: Option<i64>,
  #[serde(rename = "uuid")]
  pub uuid: String,
  #[serde(rename = "workLocationType")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub work_location_type: Option<String>,
  #[serde(rename = "workdayId")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub workday_id: Option<String>,
  #[serde(flatten)]
  #[serde(skip_serializing_if = "HashMap::is_empty")]
  pub _extra: HashMap<String, Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[remain::sorted]
pub(crate) struct Address {
  pub country: Option<String>,
  pub locality: Option<String>,
  #[serde(rename = "postalCode")]
  pub postal_code: Option<String>,
  pub state: Option<String>,
  pub street: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[remain::sorted]
pub(crate) struct OktaId {
  #[serde(rename = "Okta-B2C")]
  pub b2c: Option<String>,
  #[serde(rename = "Okta-B2E")]
  pub b2e: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[remain::sorted]
pub(crate) struct SocialId {
  #[serde(rename = "GitHub")]
  pub github: Option<String>,
}

impl TryFrom<HashMap<String, Value>> for Attributes {
  type Error = serde_json::Error;

  fn try_from(value: HashMap<String, Value>) -> Result<Self, Self::Error> {
    serde_json::from_value(serde_json::to_value(value)?)
  }
}

static CREDENTIALS: LazyLock<UserCredentials> = LazyLock::new(|| {
  UserCredentials::builder()
    .provider(
      AuthenticationProvider::builder()
        .name("AUTHENTIK")
        .type_(AuthenticationProviderType::Import)
        .build(),
    )
    .build()
});

static LINKS: LazyLock<Links> = LazyLock::new(|| {
  Links::builder()
    .self_link(
      LinksSelf::builder()
        .self_link(
          HrefObject::builder()
            .href("/")
            .type_("application/json".parse().unwrap())
            .build(),
        )
        .build(),
    )
    .build()
});

impl TryFrom<User> for OktaUser {
  type Error = serde_json::Error;

  fn try_from(
    User(AuthentikUser {
      attributes,
      avatar: _,
      date_joined,
      email,
      groups: _,
      groups_obj: _,
      is_active,
      is_superuser: _,
      last_login,
      last_updated,
      name: _,
      password_change_date,
      path: _,
      pk: _,
      roles: _,
      roles_obj: _,
      r#type: _,
      uid: _,
      username: _,
      uuid: _,
    }): User,
  ) -> Result<Self, Self::Error> {
    let Attributes {
      address,
      cn: _,
      community_uid: _,
      cost_center,
      department,
      distinguished_name: _,
      division,
      employee_number,
      employee_start_date,
      employee_status,
      employee_type,
      mut entitlements,
      given_name,
      is_manager,
      is_supervisor,
      it_stack,
      job_code,
      job_family,
      job_family_group,
      ldap_uniq: _,
      mail,
      manager_email,
      manager_uid,
      mobile,
      modify_timestamp: _,
      office,
      okta_anon_email,
      okta_anon_email_bak: _,
      okta_id,
      organization,
      secondary_workforce_manager_id,
      sn,
      social_id,
      ssh_public_key,
      telephone_number,
      title,
      uid: _,
      uid_number: _,
      upn,
      user_type,
      userdb_accounts_suse_id: _,
      userdb_login: _,
      userdb_people_suse_id: _,
      uuid: _,
      work_location_type,
      workday_id,
      _extra,
    } = attributes.unwrap_or_default().try_into()?;

    let Address {
      country,
      locality,
      postal_code,
      state,
      street,
    } = address.unwrap_or_default();

    entitlements.sort();

    let user = Self::builder()
      .created(date_joined.into())
      .credentials(CREDENTIALS.clone())
      .id(
        okta_id
          .as_ref()
          .and_then(|okta| okta.b2e.as_deref())
          .unwrap_or_default(),
      )
      .maybe_last_login(last_login.unwrap_or_default().map(Into::into))
      .last_updated(last_updated.into())
      .links(LINKS.clone())
      .password_changed(password_change_date.into())
      .profile(
        UserProfile::builder()
          .maybe_city(locality)
          .maybe_cost_center(cost_center)
          .maybe_country_code(country)
          .maybe_department(department)
          .maybe_division(division)
          .email(email.as_deref().unwrap_or("missing.email@localhost.local"))
          .employee_number(employee_number)
          .extensions(
            UserProfileExtensions::builder()
              .maybe_anon_email(okta_anon_email)
              .maybe_employee_start_date(employee_start_date)
              .maybe_employee_status(employee_status)
              .entitlement_granted(entitlements)
              .maybe_github_orgs(None)
              .maybe_github_username(social_id.map(|social_id| Vec::from_iter(social_id.github)))
              .maybe_is_manager(is_manager)
              .maybe_is_supervisor(is_supervisor)
              .maybe_it_stack(it_stack)
              .maybe_job_code(job_code)
              .maybe_job_family(job_family)
              .maybe_job_family_group(job_family_group)
              .maybe_manager_email(manager_email)
              .maybe_office(office)
              .maybe_personal_title(title.clone())
              .proxy_addresses(mail)
              .maybe_secondary_workforce_manager_id(secondary_workforce_manager_id)
              .maybe_ssh_public_key(ssh_public_key.first())
              .maybe_validated_github_orgs(None)
              .maybe_validated_github_username(None)
              .maybe_work_location_type(work_location_type)
              .maybe_workday_id(workday_id)
              .build()
              .into_opaque(),
          )
          .maybe_first_name(given_name)
          .maybe_last_name(sn)
          .login(upn)
          .maybe_manager(manager_uid)
          .maybe_mobile_phone(mobile)
          .maybe_organization(organization)
          .maybe_primary_phone(telephone_number)
          .maybe_state(state)
          .maybe_street_address(street)
          .maybe_title(title)
          .maybe_user_type(employee_type)
          .maybe_zip_code(postal_code)
          .build(),
      )
      .status(match is_active {
        None => UserStatus::Provisioned,
        Some(false) => UserStatus::Deprovisioned,
        Some(true) => UserStatus::Active,
      })
      .type_(
        UserType::builder()
          .id(user_type.as_deref().unwrap_or("converted"))
          .build(),
      )
      .build();

    Ok(user)
  }
}

pub(crate) trait TryIntoOktaUser {
  type Error: std::error::Error;

  fn try_into_okta_user(self) -> Result<OktaUser, Self::Error>;
}

impl TryIntoOktaUser for AuthentikUser {
  type Error = serde_json::Error;

  fn try_into_okta_user(self) -> Result<OktaUser, Self::Error> {
    User::from(self).try_into()
  }
}
