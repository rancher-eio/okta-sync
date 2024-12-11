use camino::Utf8PathBuf;
use clap::Parser;

#[derive(Debug, Parser)]
#[remain::sorted]
#[rustfmt::skip]
#[command(about = "generate resources from current github users")]
#[group(skip)]
pub struct Command {
  #[arg(
    long,
    help = "GitHub org to work with",
  )]
  org: String,
  #[arg(
    long,
    value_name    = "PATH",
    default_value = "members.yaml",
    help          = "file to write output to"
  )]
  output: Utf8PathBuf,
  #[arg(
    env        = "GITHUB_TOKEN",
    long       = "token",
    value_name = "TOKEN",
    help       = "GitHub Access Token",
  )]
  token: String,
}

impl Command {
  #[remain::check]
  pub async fn run(self) -> Result<(), crate::Error> {
    let Self { org, output, token } = self;

    let github = octocrab::OctocrabBuilder::new().personal_token(token).build()?;
    let members = github.all_pages(github.orgs(&org).list_members().send().await?).await?;

    let yaml = serde_yml::to_string(&members)?;

    fs_err::write(&output, &yaml)?;

    eprintln!("\n[OK] members saved to {output}");

    Ok(())
  }
}
