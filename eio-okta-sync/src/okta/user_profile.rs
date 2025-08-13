use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct UserProfileExtensions {
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "acquisitionid")]
  pub acquisition_id: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub anon_email: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub country: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "domainID")]
  pub domain_id: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub employee_start_date: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub employee_status: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub entitlement_granted: Option<Vec<String>>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub github_orgs: Option<Vec<String>>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub github_username: Option<Vec<String>>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub is_manager: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub is_supervisor: Option<bool>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub job_code: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "jobfamily")]
  pub job_family: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "jobfamilygroup")]
  pub job_family_group: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "jobfamilygroupid")]
  pub job_family_group_id: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "jobfamilyid")]
  pub job_family_id: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub legal_entity_code: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub manager_email: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub mc_opt: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub office: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub personal_title: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub proxy_addresses: Option<Vec<String>>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub secondary_employee_number: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub site_location: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub supervisory_organization: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub trello_id: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub troubleshoot: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub validated_github_orgs: Option<Vec<String>>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub validated_github_username: Option<Vec<String>>,
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "WorkLocationType")]
  pub work_location_type: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "workdayID")]
  pub workday_id: Option<String>,
}
