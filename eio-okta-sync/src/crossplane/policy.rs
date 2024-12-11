mod resolution;
mod resolve;

pub use resolution::Resolution;
pub use resolve::Resolve;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, serde::Serialize, serde::Deserialize)]
#[remain::sorted]
pub struct Policy {
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub resolution: Option<Resolution>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub resolve: Option<Resolve>,
}
