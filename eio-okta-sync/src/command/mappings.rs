use std::collections::BTreeSet;

use camino::Utf8PathBuf;
use console::style;
use eio_okta_data::current::management::components::schemas::{Group, GroupProfile, User, UserProfile};
use heck::ToKebabCase;
use inquire::{Confirm, MultiSelect, Select, Text};
use itertools::Itertools;
use rand::{Rng, rng, seq::IndexedRandom};

use crate::command::generate::{
  Expectations, Mappings, OktaGroupExpectation, OrgMapping, RoleMapping, TeamMapping, UserCriteria,
};
use crate::github::membership::Role::Admin;
use crate::okta::{Snapshot, UserProfileExtensions};

#[derive(Debug, Clone, clap::Args)]
#[remain::sorted]
#[rustfmt::skip]
#[command(about = "interactively create a mapping config")]
pub struct Command {
  #[arg(
    long,
    value_name = "BOOL",
    default_value = "true",
    action = clap::ArgAction::Set,
    help = "allow interactive prompts?",
  )]
  interactive: bool,
  #[arg(
    long,
    value_name    = "PATH",
    default_value = "snapshot.yaml",
    help          = "file to read Okta snapshot from"
  )]
  snapshot: Utf8PathBuf,
}

impl Command {
  pub fn run(self) -> Result<(), crate::Error> {
    let Self { interactive, snapshot } = self;

    let snapshot = Snapshot::read_from_file(snapshot)?;

    let mappings = Mappings {
      expectations: expectations(&snapshot, interactive)?,
      exclude_users: ignored_users(&snapshot, interactive)?,
      include_org_tree: Default::default(),
      include_users: Default::default(),
      orgs: orgs(&snapshot, interactive)?,
      roles: roles(&snapshot, interactive)?,
      teams: teams(&snapshot, interactive)?,
    };

    let yaml = serde_yml::to_string(&mappings)?;

    println!("{yaml}");

    Ok(())
  }
}

fn ignored_users(snapshot: &Snapshot, interactive: bool) -> Result<UserCriteria, crate::Error> {
  if interactive
    && Confirm::new("Do you want to ignore any users?")
      .with_default(false)
      .prompt()?
  {
    Ok(UserCriteria {
      github_usernames: ignored_users_by_github_username(snapshot, interactive)?,
      okta_ids: Default::default(),
      okta_profile_emails: ignored_users_by_okta_profile_email(snapshot, interactive)?,
    })
  } else {
    Ok(UserCriteria::default())
  }
}

fn ignored_users_by_github_username(snapshot: &Snapshot, interactive: bool) -> Result<Vec<String>, crate::Error> {
  if interactive
    && Confirm::new("Ignore users by GitHub Username?")
      .with_default(false)
      .prompt()?
  {
    Ok(
      MultiSelect::new(
        "Which GitHub Usernames should be ignored?",
        snapshot.users.github_users()?.github_usernames()?,
      )
      .prompt()?,
    )
  } else {
    Ok(Vec::new())
  }
}

fn ignored_users_by_okta_profile_email(snapshot: &Snapshot, interactive: bool) -> Result<Vec<String>, crate::Error> {
  if interactive
    && Confirm::new("Ignore users by Okta profile email?")
      .with_default(false)
      .prompt()?
  {
    Ok(
      MultiSelect::new(
        "Which user emails should be ignored?",
        snapshot.users.github_users()?.user_emails(),
      )
      .prompt()?,
    )
  } else {
    Ok(Vec::new())
  }
}

fn orgs(snapshot: &Snapshot, interactive: bool) -> Result<Vec<OrgMapping>, crate::Error> {
  let mut mappings = Vec::new();

  if interactive {
    let mut github_orgs = snapshot.users.github_users()?.github_orgs()?.into_iter().collect_vec();

    if !github_orgs.is_empty() {
      eprintln!(
        "found {} GitHub Orgs referenced in Okta User Profiles...",
        github_orgs.len()
      );
      if Confirm::new("Do you want to map these to Org Memberships?")
        .with_default(true)
        .prompt()?
      {
        github_orgs = MultiSelect::new("Which of these GitHub Orgs (as they appear in Profiles)?", github_orgs)
          .with_all_selected_by_default()
          .prompt()?;

        for okta_profile_github_org in github_orgs {
          let mut github_org_name = okta_profile_github_org.to_kebab_case();
          let message = format!(
            "Okta Users whose .profile.githubOrgs[] contains '{okta_profile_github_org}' will be members of which Org on GitHub?"
          );
          github_org_name = Text::new(&message).with_initial_value(&github_org_name).prompt()?;

          mappings.push(OrgMapping {
            github_org_name,
            okta_profile_github_org,
          });
        }
      }
    }
  }

  Ok(mappings)
}

fn roles(snapshot: &Snapshot, interactive: bool) -> Result<Vec<RoleMapping>, crate::Error> {
  let mut roles = Vec::new();

  if interactive {
    if Confirm::new("Would you like to assign GitHub Roles based on Okta Group Membership?")
      .with_default(true)
      .prompt()?
    {
      for group in MultiSelect::new("Which Okta Groups?", snapshot.groups.to_okta_group_expectations()).prompt()? {
        if let Some(users) = snapshot.group_users.get(&group.id) {
          let emails = users.github_users()?.user_emails();

          for okta_profile_email in MultiSelect::new("Assign Admin (Owner) role to which users?", emails)
            .with_all_selected_by_default()
            .prompt()?
          {
            roles.push(RoleMapping {
              github_member_role: Admin,
              okta_profile_email,
            })
          }
        }
      }
    }

    if Confirm::new("Would you like to assign GitHub Roles to any individual Okta Users?")
      .with_default(true)
      .prompt()?
    {
      let emails = snapshot.users.github_users()?.user_emails();

      for okta_profile_email in MultiSelect::new("Assign Admin (Owner) role to which users?", emails).prompt()? {
        roles.push(RoleMapping {
          github_member_role: Admin,
          okta_profile_email,
        })
      }
    }
  }

  Ok(roles.into_iter().unique().collect())
}

fn teams(snapshot: &Snapshot, interactive: bool) -> Result<Vec<TeamMapping>, crate::Error> {
  let mut teams = Vec::new();

  if interactive
    && Confirm::new("Would you like to map any Okta Groups to GitHub Teams?")
      .with_default(true)
      .prompt()?
  {
    let groups = MultiSelect::new(
      "Which Okta Groups should map to GitHub Teams?",
      snapshot.groups.to_okta_group_expectations(),
    )
    .prompt()?;

    for group in groups {
      let OktaGroupExpectation {
        id: okta_group_id,
        profile_name,
      } = group;
      let mut github_team_name = profile_name.to_kebab_case();

      let message = format!("Okta Group '{profile_name}' maps to which GitHub Team?");
      github_team_name = Text::new(&message).with_initial_value(&github_team_name).prompt()?;

      teams.push(TeamMapping {
        github_team_name,
        okta_group_id,
      });
    }
  }

  Ok(teams)
}

fn expectations(snapshot: &Snapshot, interactive: bool) -> Result<Expectations, crate::Error> {
  let mut okta_groups = Vec::new();

  eprintln!("Let's start with {} ...", style("expectations").bold());

  let groups = snapshot.groups.to_okta_group_expectations();

  eprintln!("Okta Group IDs are opaque, which is a recipe for misconfiguration.");

  if interactive {
    let mut rng = rng();

    if Confirm::new("Would you like a demonstration of the problem, using your snapshot data?")
      .with_default(true)
      .prompt()?
    {
      quiz(&groups, &mut rng)?;
    }
  }

  eprintln!(
    "\nTo mitigate this, we can indicate what we expect the profile name to be, this can be used to check these assumptions in future runs."
  );

  if interactive {
    okta_groups = inquire::MultiSelect::new("Which Okta Groups do you want to use?", groups)
      .with_all_selected_by_default()
      .prompt()?;
  }

  Ok(Expectations { okta_groups })
}

const SKIP_QUIZ_MESSAGE: &str = "┬─┬ノ( º _ ºノ) fair enough, we can skip that...";

fn quiz(groups: &[OktaGroupExpectation], rng: &mut impl Rng) -> Result<(), crate::Error> {
  if groups.len() > 1 {
    eprintln!("\n🤔💡 the problem can be illustrated with two questions (you are not required to answer correctly):");

    match quiz_group_id(groups, rng)? {
      None => eprintln!("{SKIP_QUIZ_MESSAGE}"),
      Some(_ids) => match quiz_group_name(groups, rng)? {
        None => eprintln!("{SKIP_QUIZ_MESSAGE}"),
        Some(_names) => match confirm_understanding()? {
          None => eprintln!("{SKIP_QUIZ_MESSAGE}"),
          Some(understood) => {
            if !understood {
              quiz(groups, rng)?
            }
          }
        },
      },
    }
  } else {
    eprintln!("\nYou only have one group, so this may not seem like a problem... yet.")
  }
  Ok(())
}

const UNREACHABLE_ERROR: &str = "(╯°□°)╯︵ ┻━┻
䷓ This error was thought to be unreachable, and yet... here we are.
䷴ Use this time to reflect on whatever holds you back from your dreams.
䷼ How sure are you that obstacles are immovable, or problems unsolvable?
䷪ Cast off the shackles of your mind and be free, like this program.";

fn confirm_understanding() -> Result<Option<bool>, crate::Error> {
  Confirm::new("Do you see the problem now?")
    .with_default(false)
    .prompt_skippable()
    .map_err(Into::into)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct QuizItemGroupId<'expectation>(&'expectation OktaGroupExpectation);
impl std::fmt::Display for QuizItemGroupId<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.0.id.fmt(f)
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct QuizItemGroupName<'expectation>(&'expectation OktaGroupExpectation);
impl std::fmt::Display for QuizItemGroupName<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.0.profile_name.fmt(f)
  }
}

impl<'expectation> From<QuizItemGroupId<'expectation>> for QuizItemGroupName<'expectation> {
  fn from(value: QuizItemGroupId<'expectation>) -> Self {
    Self(value.0)
  }
}

impl<'expectation> From<QuizItemGroupName<'expectation>> for QuizItemGroupId<'expectation> {
  fn from(value: QuizItemGroupName<'expectation>) -> Self {
    Self(value.0)
  }
}

fn quiz_group_id(groups: &[OktaGroupExpectation], rng: &mut impl Rng) -> Result<Option<bool>, crate::Error> {
  let groups = groups.iter().map(QuizItemGroupId).collect_vec();

  let amount = 5.min(groups.len());
  let examples = groups.choose_multiple(rng, amount).collect_vec();

  match examples.choose(rng) {
    Some(expected) => {
      let prompt = format!("😏 What is the group ID for '{}'", expected.0.profile_name.as_str());

      match Select::new(&prompt, examples.clone()).prompt_skippable()? {
        None => Ok(None),
        Some(selected) if selected == *expected => {
          eprintln!("Correct! Did you look it up, was it a lucky guess, or did you actually know the answer?");
          Ok(Some(true))
        }
        Some(selected) => {
          let id = style(selected).red();
          let name = style(QuizItemGroupName(selected.0)).red();
          let answer = style(expected).green();

          eprintln!("'{id}' is actually '{name}'... '{answer}' was the correct answer.",);
          Ok(Some(false))
        }
      }
    }
    None => {
      eprintln!("{UNREACHABLE_ERROR}");
      std::process::exit(420);
    }
  }
}

fn quiz_group_name(groups: &[OktaGroupExpectation], rng: &mut impl Rng) -> Result<Option<bool>, crate::Error> {
  let groups = groups.iter().map(QuizItemGroupName).collect_vec();

  let amount = 5.min(groups.len());
  let examples = groups.choose_multiple(rng, amount).collect_vec();

  match examples.choose(rng) {
    Some(expected) => {
      let prompt = format!("😏 Which group has ID '{}'?", expected.0.id.as_str());

      match Select::new(&prompt, examples.clone()).prompt_skippable()? {
        None => Ok(None),
        Some(selected) if selected == *expected => {
          eprintln!("Correct! Did you look it up, was it a lucky guess, or did you actually know the answer?");
          Ok(Some(true))
        }
        Some(selected) => {
          let id = style(QuizItemGroupId(selected.0)).red();
          let name = style(selected).red();
          let answer = style(expected).green();

          eprintln!("'{name}' has ID '{id}'... '{answer}' was the correct answer.");
          Ok(Some(false))
        }
      }
    }
    None => {
      eprintln!("{UNREACHABLE_ERROR}");
      std::process::exit(420);
    }
  }
}

trait ToOktaGroupExpectations {
  fn to_okta_group_expectations(&self) -> Vec<OktaGroupExpectation>;
}

impl ToOktaGroupExpectations for [Group] {
  fn to_okta_group_expectations(&self) -> Vec<OktaGroupExpectation> {
    self
      .iter()
      .map(
        |Group {
           id,
           profile: GroupProfile { name, .. },
           ..
         }| {
          OktaGroupExpectation {
            id: id.clone(),
            profile_name: name.clone(),
          }
        },
      )
      .collect()
  }
}

trait GithubUsers {
  fn github_users(&self) -> Result<Vec<&User>, crate::Error>;
}

impl GithubUsers for [User] {
  fn github_users(&self) -> Result<Vec<&User>, crate::Error> {
    let mut users = Vec::new();

    for user in self {
      if user
        .profile
        .extensions_into::<UserProfileExtensions>()?
        .github_username
        .is_some_and(|username| !username.is_empty())
      {
        users.push(user)
      }
    }

    Ok(users)
  }
}

trait GitHubUsernames {
  fn github_usernames(&self) -> Result<Vec<String>, crate::Error>;
}

impl GitHubUsernames for [&User] {
  fn github_usernames(&self) -> Result<Vec<String>, crate::Error> {
    let mut usernames = Vec::new();

    for user in self {
      if let Some(github) = user.profile.extensions_into::<UserProfileExtensions>()?.github_username {
        for username in github {
          usernames.push(username);
        }
      }
    }

    Ok(usernames)
  }
}

trait UserEmails {
  fn user_emails(&self) -> Vec<String>;
}

impl UserEmails for [&User] {
  fn user_emails(&self) -> Vec<String> {
    self
      .iter()
      .map(
        |User {
           profile: UserProfile { email, .. },
           ..
         }| email,
      )
      .sorted_unstable()
      .map(ToOwned::to_owned)
      .collect()
  }
}

trait GithubOrgs {
  fn github_orgs(&self) -> Result<BTreeSet<String>, crate::Error>;
}

impl GithubOrgs for [&User] {
  fn github_orgs(&self) -> Result<BTreeSet<String>, crate::Error> {
    let mut orgs = BTreeSet::new();

    for user in self {
      if let Some(github) = user.profile.extensions_into::<UserProfileExtensions>()?.github_orgs {
        for org in github {
          orgs.insert(org);
        }
      }
    }

    Ok(orgs)
  }
}
