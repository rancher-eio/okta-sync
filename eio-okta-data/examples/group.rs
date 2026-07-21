use eio_okta_data::v2024_07_0::management::components::schemas::Group;
use fake::{Fake, Faker};

fn main() -> Result<(), serde_saphyr::ser::Error> {
  let fake = Faker.fake::<Group>();
  let yaml = serde_saphyr::to_string(&fake)?;
  println!("{yaml}");
  Ok(())
}
