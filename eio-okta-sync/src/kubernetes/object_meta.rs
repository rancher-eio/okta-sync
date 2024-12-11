use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[remain::sorted]
pub struct ObjectMeta {
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub annotations: Option<HashMap<String, String>>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub labels: Option<HashMap<String, String>>,
  pub name: String,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub namespace: Option<String>,
}
