#[crate::apply(crate::structs!)]
pub struct UserType {
  pub id: String,
}

crate::impl_as_ref!(UserType);

#[cfg(test)]
mod tests {
  crate::testing::roundtrip!(super::UserType);
  crate::testing::validate!(super::UserType);
}
