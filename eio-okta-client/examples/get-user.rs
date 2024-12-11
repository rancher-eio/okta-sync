use clap::Parser;
use eio_okta_api::v1::users::GetUser;
use eio_okta_client::command::Standalone;

fn main() {
  let json = Standalone::<GetUser>::parse().resource_json().unwrap();

  println!("{json}");
}
