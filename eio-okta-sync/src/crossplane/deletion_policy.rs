#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[remain::sorted]
pub enum DeletionPolicy {
  Delete,
  Orphan,
}
