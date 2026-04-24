use crate::github::enterprise::models::OrganizationSelectionType;
use crate::github::Enterprise;
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

  #[arg(
    env        = "GITHUB_TOKEN",
    long       = "token",
    value_name = "TOKEN",
    help       = "GitHub Access Token",
    hide_env_values = true,
  )]
  token: String,
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

    let result = match description {
      Some(desc) => {
        github
          .enterprise(&enterprise_slug)
          .teams()
          .create(&name)
          .description(desc)
          .organization_selection_type(OrganizationSelectionType::Selected)
          .build()
          .send()
          .await?
      }
      None => {
        github
          .enterprise(&enterprise_slug)
          .teams()
          .create(&name)
          .organization_selection_type(OrganizationSelectionType::Selected)
          .build()
          .send()
          .await?
      }
    };

    let json = serde_json::to_string_pretty(&result)?;
    println!("{json}");

    Ok(())
  }
}
