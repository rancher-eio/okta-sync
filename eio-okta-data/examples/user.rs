use eio_okta_data::v2024_07_0::management::components::schemas::User;
use fake::{Fake, Faker};

fn main() -> serde_yml::Result<()> {
  let fake = Faker.fake::<User>();
  let yaml = serde_yml::to_string(&fake)?;
  println!("{yaml}");
  Ok(())
}
