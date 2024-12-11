#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[remain::sorted]
pub struct Resource<T> {
  pub api_version: String,
  pub kind: String,
  pub metadata: super::ObjectMeta,
  pub spec: T,
}
