use std::collections::BTreeSet;

use bounded_integer::BoundedU8;
use camino::Utf8PathBuf;
use itertools::Itertools;
use octocrab::Octocrab;
use petgraph::visit::NodeIndexable;

use crate::{
  github::Enterprise,
  okta::{DisplayName, Snapshot, UserProfileExtensions, graph::Org},
};

#[derive(Debug, clap::Parser)]
#[remain::sorted]
#[rustfmt::skip]
#[command(about = "assign members to an Enterprise Team based on Okta snapshot data")]
pub struct Command {
  #[arg(
    long,
    value_name     = "BOOL",
    default_value  = "true",
    action         = clap::ArgAction::Set,
    help           = "create team if not found?"
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
    help          = "name of enterprise to use"
  )]
  enterprise: String,
  #[arg(
    long,
    value_name    = "SLUG",
    default_value = "rancher",
    help          = "the name of the org to use",
  )]
  org: String,
  #[arg(
    long,
    value_name = "OKTA-ID",
    help       = "find okta users starting from (but not including) this user (defaults to top of org)",
  )]
  root: Option<String>,
  #[arg(
    long,
    value_name = "PATH",
    default_value = "snapshot.yaml",
    help = "file to read Okta snapshot from"
  )]
  snapshot: Utf8PathBuf,
  #[arg(
    long,
    value_name     = "ID",
    conflicts_with = "team_name",
    help           = "the ID of an existing Enterprise Team to assign users from",
  )]
  team_id: Option<String>,
  #[arg(
    long,
    value_name = "STRING",
    help       = "the name of Enterprise Team to assign users from (defaults to org name)",
  )]
  team_name: Option<String>,
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
      create_missing,
      dry_run,
      enterprise,
      token,
      org,
      root,
      snapshot,
      team_id,
      team_name,
    } = self;

    let github = Octocrab::builder().personal_token(token).build()?;

    let per_page = BoundedU8::const_new::<100>();

    let enterprise = github.enterprise(&enterprise);

    let team = match team_id {
      Some(id) => {
        eprintln!("finding team with ID '{id}...");
        enterprise.team(&id).get().send().await?
      }
      None => {
        let name = team_name.unwrap_or_else(|| {
          eprintln!("no team name given, defaulting to org name: '{org}'");
          org.clone()
        });

        eprintln!("finding team with name '{name}'...");
        match github
          .all_pages(enterprise.teams().list().per_page(per_page).build().send().await?)
          .await?
          .into_iter()
          .find(|team| team.name.eq_ignore_ascii_case(&name))
        {
          Some(team) => team,
          None => {
            if create_missing {
              eprintln!("team with name '{name}' does not exist, creating...");
              if dry_run {
                eprintln!("aborting (dry-run)");
                std::process::exit(0);
              } else {
                enterprise.teams().create(&name).build().send().await?
              }
            } else {
              eprintln!("team with name '{name}' does not exist, aborting...");
              std::process::exit(1);
            }
          }
        }
      }
    };

    eprintln!("team exists: '{}' ({})", &team.slug, &team.id);

    eprintln!("loading Okta snapshot: {snapshot}");
    let snapshot = Snapshot::read_from_file(snapshot)?;

    eprintln!("constructing org graph from snapshot...");
    let okta_org = Org::new(&snapshot.users).populate_descending();

    eprintln!("org graph contains {} users", okta_org.hierarchy.node_count());

    let root_id = root.unwrap_or_else(|| {
      eprintln!("no root given, defaulting to top of org");
      okta_org.hierarchy.from_index(0).to_owned()
    });

    let root = okta_org
      .user(&root_id)
      .expect(&format!("failed to find user in org graph with ID '{root_id}'"));

    eprintln!(
      "finding users in org graph from user '{}' ({})...",
      &root.id,
      root.display_name()
    );

    let okta_users = okta_org
      .below(&root.id)
      .iter()
      .flat_map(|id| okta_org.user(id))
      .flat_map(|user| user.profile.extensions_into::<UserProfileExtensions>().ok())
      .flat_map(|profile| profile.github_username.unwrap_or_default())
      .map(|username| username.to_lowercase())
      .collect::<BTreeSet<String>>();

    eprintln!("found {} GitHub usernames in relevant Okta profiles", okta_users.len());

    eprintln!("finding all users in GitHub Org '{org}'...");
    let org_members = github
      .all_pages(github.orgs(&org).list_members().per_page(per_page).send().await?)
      .await?
      .into_iter()
      .map(|member| member.login.to_lowercase())
      .collect::<BTreeSet<String>>();

    eprintln!("found {} users in GitHub Org '{org}'", org_members.len());

    let matched_users = okta_users.intersection(&org_members).cloned().collect::<BTreeSet<_>>();

    eprintln!("users in both Okta and GitHub Org: {}", matched_users.len());

    eprintln!("finding all users in Enterprise Team '{}'", &team.slug);
    let enterprise_team_members = github
      .all_pages(
        enterprise
          .team(&team.slug)
          .memberships()
          .list()
          .per_page(per_page)
          .build()
          .send()
          .await?,
      )
      .await?
      .into_iter()
      .map(|member| member.login.to_lowercase())
      .collect::<BTreeSet<String>>();

    eprintln!(
      "found {} users in Enterprise Team '{}'",
      enterprise_team_members.len(),
      &team.slug
    );

    let missing_users = matched_users
      .difference(&enterprise_team_members)
      .cloned()
      .collect_vec();

    if missing_users.is_empty() {
      eprintln!("team '{}' already contains all expected users", &team.slug);
    } else {
      let total = missing_users.len();
      for chunk in missing_users.chunks(100) {
        eprintln!(
          "adding {} of {} missing users to team {}",
          chunk.len(),
          total,
          &team.slug
        );

        if dry_run {
          eprintln!("skipping (dry-run)");
        } else {
          enterprise
            .team(&team.slug)
            .memberships()
            .bulk_add()
            .usernames(chunk.into())
            .build()
            .send()
            .await?;
        }
      }
    }

    Ok(())
  }
}
