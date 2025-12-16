use clap::Parser;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> miette::Result<()> {
  tracing_subscriber::registry()
    .with(EnvFilter::from_default_env())
    .init();

  eio_okta_sync::command::Command::parse().run().await?;

  Ok(())
}
