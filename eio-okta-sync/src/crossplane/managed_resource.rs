use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[remain::sorted]
pub struct ManagedResource<T> {
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub deletion_policy: Option<super::DeletionPolicy>,
  pub for_provider: T,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub init_provider: Option<T>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub management_policies: Option<Value>,
  pub provider_config_ref: super::ProviderConfigReference,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub publish_connection_details_to: Option<super::PublishConnectionDetailsTo>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub write_connection_secret_to_ref: Option<super::WriteConnectionSecretToReference>,
}
