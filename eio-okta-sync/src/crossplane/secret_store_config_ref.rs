#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[remain::sorted]
pub struct SecretStoreConfigRef {
  pub name: String,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub policy: Option<super::Policy>,
}
