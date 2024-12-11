#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[remain::sorted]
pub struct PublishConnectionDetailsTo {
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub config_ref: Option<super::SecretStoreConfigRef>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub metadata: Option<super::ConnectionSecretMetadata>,
  pub name: String,
}
