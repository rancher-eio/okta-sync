#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[remain::sorted]
pub enum Resolution {
  Optional,
  #[default]
  Required,
}
