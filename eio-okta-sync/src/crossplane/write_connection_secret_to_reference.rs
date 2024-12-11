#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[remain::sorted]
pub struct WriteConnectionSecretToReference {
  pub name: String,
  pub namespace: String,
}
