#[crate::apply(crate::structs_with_args!)]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct PathUserId {
  #[cfg_attr(feature = "clap", arg(long, value_name = "ID or LOGIN", help = "ID of an existing Okta user"))]
  pub user_id: String,
}

#[cfg(test)]
mod tests {
  crate::testing::roundtrip!(super::PathUserId);
  crate::testing::validate!(super::PathUserId);
}
