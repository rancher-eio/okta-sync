use clap::Parser;

fn main() -> miette::Result<()> {
  eio_okta_client::Command::parse().run()?;

  Ok(())
}
