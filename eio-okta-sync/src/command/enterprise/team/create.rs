use crate::github::Enterprise;
use crate::github::enterprise::models::OrganizationSelectionType;
use octocrab::Octocrab;

#[derive(Debug, clap::Parser)]
#[remain::sorted]
#[rustfmt::skip]
#[command(about = "create an enterprise team")]
pub struct Command {
  #[arg(
    long,
    value_name = "STRING",
    help       = "team description (optional)"
  )]
  description: Option<String>,

  #[arg(
    long,
    value_name    = "SLUG",
    default_value = "suse-gmbh",
    help          = "name of enterprise to use"
  )]
  enterprise_slug: String,

  #[arg(
    long,
    value_name = "NAME",
    help       = "team name"
  )]
  name: String,

  #[command(flatten)]
  token: crate::args::github::Token,
}

impl Command {
  pub async fn run(self) -> Result<(), crate::Error> {
    let Self {
      description,
      enterprise_slug,
      name,
      token,
    } = self;

    let github = Octocrab::builder().personal_token(token).build()?;
    let result = github
      .enterprise(&enterprise_slug)
      .teams()
      .create(&name)
      .maybe_description(description)
      .organization_selection_type(OrganizationSelectionType::Selected)
      .build()
      .send()
      .await?;
    let json = serde_json::to_string_pretty(&result)?;
    println!("{json}");

    Ok(())
  }
}
