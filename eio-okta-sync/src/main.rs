use clap::Parser;

#[tokio::main]
async fn main() -> miette::Result<()> {
  eio_okta_sync::command::Command::parse().run().await?;

  Ok(())
}
