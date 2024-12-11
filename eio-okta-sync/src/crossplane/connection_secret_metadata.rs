use std::collections::BTreeMap;

use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, serde::Serialize, serde::Deserialize)]
#[remain::sorted]
pub struct ConnectionSecretMetadata {
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub annotations: Option<BTreeMap<String, Value>>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub labels: Option<BTreeMap<String, Value>>,
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
  pub type_: Option<String>,
}
