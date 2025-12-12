use bounded_integer::BoundedU8;
use itertools::Itertools;
use octocrab::Octocrab;
use uuid::Uuid;

use crate::github::Enterprise;

#[derive(Debug, clap::Parser)]
#[remain::sorted]
#[rustfmt::skip]
#[command(about = "assign all members of an Enterprise Team to a Cost Center")]
pub struct Command {
  #[arg(
    long,
    value_name     = "UUID",
    help           = "the ID of an existing Cost Center to use",
  )]
  cost_center_id: Option<Uuid>,
  #[arg(
    long,
    value_name     = "STRING",
    conflicts_with = "cost_center_id",
    help           = "the name of a Cost Center, defaults to team name"
  )]
  cost_center_name: Option<String>,
  #[arg(
    long,
    value_name     = "BOOL",
    default_value  = "false",
    action         = clap::ArgAction::Set,
    conflicts_with = "cost_center_id",
    help           = "create Cost Center if none match name?"
  )]
  create_missing: bool,
  #[arg(
    long,
    value_name    = "BOOL",
    default_value = "true",
    action        = clap::ArgAction::Set,
    help          = "do nothing, only print what would have been done"
  )]
  dry_run: bool,
  #[arg(
    long,
    value_name    = "SLUG",
    default_value = "suse-gmbh",
    help          = "name of Enterprise to use"
  )]
  enterprise: String,
  #[arg(
    long,
    value_name = "SLUG",
    help       = "name of Enterprise Team to assign users from",
  )]
  team: String,
  #[arg(
    env        = "GITHUB_TOKEN",
    long       = "token",
    value_name = "TOKEN",
    help       = "GitHub Access Token",
  )]
  token: String,
}

impl Command {
  pub async fn run(self) -> Result<(), crate::Error> {
    let Self {
      cost_center_id,
      cost_center_name,
      create_missing,
      dry_run,
      enterprise,
      token,
      team,
    } = self;

    let github = Octocrab::builder().personal_token(token).build()?;
    let per_page = BoundedU8::const_new::<100>();

    let enterprise = github.enterprise(&enterprise);

    let cost_center = {
      match cost_center_id {
        Some(cost_center_id) => {
          eprintln!("finding cost center with id '{cost_center_id}'...");
          enterprise.cost_center(&cost_center_id).get().send().await?
        }
        None => {
          let name = cost_center_name.unwrap_or_else(|| {
            eprintln!("no cost center name given, defaulting to team name: '{team}'");
            team.clone()
          });

          eprintln!("finding cost center with name '{name}'...");

          match enterprise
            .cost_centers()
            .list()
            .build()
            .send()
            .await?
            .cost_centers
            .into_iter()
            .find(|cc| cc.name.eq_ignore_ascii_case(&name))
          {
            Some(cost_center) => cost_center,
            None => {
              if create_missing {
                eprintln!("cost center with name '{name}' does not exist, creating...");
                if dry_run {
                  eprintln!("aborting (dry-run)");
                  std::process::exit(0);
                } else {
                  enterprise.cost_centers().create(&name).send().await?
                }
              } else {
                eprintln!("cost center with name '{name}' does not exist, aborting...");
                std::process::exit(1);
              }
            }
          }
        }
      }
    };

    eprintln!("cost center exists: {}", &cost_center.id);

    let users = github
      .all_pages(
        enterprise
          .team(&team)
          .memberships()
          .list()
          .per_page(per_page)
          .build()
          .send()
          .await?,
      )
      .await?
      .into_iter()
      .map(|member| member.login)
      .collect_vec();

    let total = users.len();

    for chunk in users.chunks(50) {
      eprintln!(
        "assigning {} of {} users to cost center {} ({})...",
        chunk.len(),
        total,
        &cost_center.id,
        &cost_center.name
      );

      if dry_run {
        eprintln!("skipping (dry-run)");
      } else {
        let value = enterprise
          .cost_center(&cost_center.id)
          .add_resources()
          .users(chunk.into())
          .build()
          .send()
          .await?;

        println!("{json}", json = serde_json::to_string_pretty(&value)?);
      }
    }

    Ok(())
  }
}
