use crate::authentik::traits::RunAsync;

#[derive(Debug, Clone, clap::Args)]
#[command(about = "experimental support for Authentik")]
pub struct Command {
  #[command(subcommand)]
  subcommand: crate::authentik::Command,
}

impl Command {
  pub(crate) async fn run(self) -> Result<(), crate::Error> {
    Ok(self.subcommand.run().await?)
  }
}
