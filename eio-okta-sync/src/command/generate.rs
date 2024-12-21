use crate::crossplane::ProviderConfigReference;
use crate::okta::Snapshot;
use crate::{crossplane, github, kubernetes};
use camino::Utf8PathBuf;
use fs_err::File;
use std::collections::{HashMap, HashSet};
use std::io::Read;

#[derive(Debug, Clone, PartialEq, Eq, clap::Args)]
#[remain::sorted]
#[rustfmt::skip]
#[command(about = "generate GitHub resources from Okta snapshot")]
pub struct Command {
  #[command(flatten)]
  embed_github_org_name: EmbedGithubOrgName,
  #[arg(
    long,
    value_name    = "PATH",
    default_value = "mappings.yaml",
    help      = "mapping config file for Okta Groups to Orgs and Teams (--help for example)",
    long_help = "mapping config file for Okta Groups to Orgs and Teams.
# Example Config
---
expectations:              # Okta Group IDs are opaque. By indicating the expected profile name, the program can check our assumptions.
  oktaGroups:              ########## groups to expect in the snapshot
    - id:                  # [Okta]     the Group ID
      profileName:         # [Okta]     the expected name of the profile
roles:                     ########## users to assign a role to
  - oktaProfileEmail:      # [Okta]     the email address of the user
    githubMemberRole:      # [GitHub]   the role of the member
teams:                     ########## teams to create resources for
  - githubTeamName:        # [GitHub]   the team slug
    oktaGroupId:           # [Okta]     the Group ID
orgs:                      ########## orgs to create resources for
  - githubOrgName:         # [GitHub]   the org name
    oktaProfileGithubOrg:  # [Okta]     value in .profile.githubOrgs
    teams:                 ##########   teams to create for this org
    - githubTeamName:      # [GitHub]     the team slug
      oktaGroupId:         # [Okta]       the Group ID
"
  )]
  mappings: Utf8PathBuf,
  #[arg(
    long,
    value_name    = "PATH",
    default_value = "resources.yaml",
    help          = "file to write output to"
  )]
  output: Utf8PathBuf,
  #[arg(
    long,
    value_name    = "PATH",
    default_value = "snapshot.yaml",
    help          = "file to read Okta snapshot from"
  )]
  snapshot: Utf8PathBuf,
}

const DEFAULT_ORG_ANNOTATION: &str = "eio.rancher.engineering/github-org-name";
const DEFAULT_ORG_LABEL: &str = "eio.rancher.engineering/github-org-name";

#[derive(Debug, Clone, PartialEq, Eq, Hash, clap::Args)]
#[group(id = "embed.github.org.name")]
#[remain::sorted]
#[rustfmt::skip]
pub struct EmbedGithubOrgName {
  #[arg(
    env  = "EMBED_GITHUB_ORG_NAME_ANNOTATION",
    long = "embed-github-org-name-annotation",
    id   = "embed.github.org.name.annotation",
    value_name    = "KEY",
    default_value = DEFAULT_ORG_ANNOTATION,
    help_heading = "Embedding Options (GitHub Org Name)",
    help         = "annotation to use when --embed-github-org-name='annotation'"
  )]
  pub(crate) annotation: String,
  #[arg(
    env  = "EMBED_GITHUB_ORG_NAME_DELIMITER",
    long = "embed-github-org-name-delimiter",
    id   = "embed.github.org.name.delimiter",
    value_name    = "DELIMITER",
    default_value = "--",
    help_heading = "Embedding Options (GitHub Org Name)",
    help         = "delimiter to use when --embed-github-org-name='name-prefix'"
  )]
  pub(crate) delimiter: String,
  #[arg(
    env  = "EMBED_GITHUB_ORG_NAME_LABEL",
    long = "embed-github-org-name-label",
    id   = "embed.github.org.name.label",
    value_name    = "KEY",
    default_value = DEFAULT_ORG_LABEL,
    help_heading = "Embedding Options (GitHub Org Name)",
    help         = "label to use when --embed-github-org-name='label'"
  )]
  pub(crate) label: String,
  #[arg(
    env  = "EMBED_GITHUB_ORG_NAME",
    long = "embed-github-org-name",
    id   = "embed.github.org.name.target",
    value_name    = "TARGET",
    default_value = "name-prefix",
    help_heading = "Embedding Options (GitHub Org Name)",
    help         = "where to embed github org name",
    long_help    = "where to embed github org name.

- annotation  = embeds in $.metadata.annotation[EMBED_GITHUB_ORG_NAME_ANNOTATION]
- label       = embeds in $.metadata.label[EMBED_GITHUB_ORG_NAME_LABEL]
- name-prefix = embeds as prefix to $.metadata.name with EMBED_GITHUB_ORG_NAME_DELIMITER
- namespace   = embeds as $.metadata.namespace (regardless of if resources is namespaced)
- none        = does not embed anywhere"
  )]
  pub(crate) target: EmbedTarget,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, clap::ValueEnum)]
#[clap(rename_all = "kebab-case")]
#[remain::sorted]
pub(crate) enum EmbedTarget {
  Annotation,
  Label,
  NamePrefix,
  Namespace,
  None,
}

impl EmbedGithubOrgName {
  pub(crate) fn embed(&self, org_name: String, metadata: &mut kubernetes::ObjectMeta) {
    let Self {
      annotation,
      delimiter,
      label,
      target,
    } = self;

    match target {
      EmbedTarget::Annotation => {
        metadata
          .annotations
          .get_or_insert_default()
          .insert(annotation.to_owned(), org_name);
      }
      EmbedTarget::Label => {
        metadata
          .labels
          .get_or_insert_default()
          .insert(label.to_owned(), org_name);
      }
      EmbedTarget::NamePrefix => metadata.name.insert_str(0, &format!("{org_name}{delimiter}")),
      EmbedTarget::Namespace => {
        metadata.namespace = Some(org_name);
      }
      EmbedTarget::None => (),
    };
  }

  pub(crate) fn org<'metadata>(&self, metadata: &'metadata kubernetes::ObjectMeta) -> Option<&'metadata str> {
    let Self {
      annotation,
      delimiter,
      label,
      target,
    } = self;

    match target {
      EmbedTarget::Annotation => metadata
        .annotations
        .as_ref()
        .and_then(|annotations| annotations.get(annotation))
        .map(AsRef::as_ref),
      EmbedTarget::Label => metadata
        .labels
        .as_ref()
        .and_then(|labels| labels.get(label))
        .map(AsRef::as_ref),
      EmbedTarget::NamePrefix => metadata.name.split_once(delimiter).map(|(org_name, _)| org_name),
      EmbedTarget::Namespace => metadata.namespace.as_ref().map(AsRef::as_ref),
      EmbedTarget::None => None,
    }
  }

  pub(crate) fn name<'metadata>(&self, metadata: &'metadata kubernetes::ObjectMeta) -> &'metadata str {
    let Self { delimiter, target, .. } = self;

    if let EmbedTarget::NamePrefix = target {
      metadata
        .name
        .split_once(delimiter)
        .map(|(_, name)| name)
        .unwrap_or_else(|| metadata.name.as_ref())
    } else {
      metadata.name.as_ref()
    }
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Mappings {
  pub(crate) expectations: Expectations,
  pub(crate) orgs: Vec<OrgMapping>,
  pub(crate) roles: Vec<RoleMapping>,
  pub(crate) teams: Vec<TeamMapping>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Expectations {
  pub(crate) okta_groups: Vec<OktaGroupExpectation>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct OktaGroupExpectation {
  pub(crate) id: String,
  pub(crate) profile_name: String,
}

impl std::fmt::Display for OktaGroupExpectation {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.profile_name.fmt(f)
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct OrgMapping {
  pub(crate) github_org_name: String,
  pub(crate) okta_profile_github_org: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RoleMapping {
  pub(crate) github_member_role: github::membership::Role,
  pub(crate) okta_profile_email: String,
}

impl std::fmt::Display for RoleMapping {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.okta_profile_email.fmt(f)
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct TeamMapping {
  pub(crate) github_team_name: String,
  pub(crate) okta_group_id: String,
}

impl std::fmt::Display for TeamMapping {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.github_team_name.fmt(f)
  }
}

const LABEL_OKTA_GROUP_ID: &str = "suse.okta.com/group-id";
const LABEL_OKTA_USER_ID: &str = "suse.okta.com/user-id";

impl Command {
  pub fn run(&self) -> Result<(), crate::Error> {
    let Self {
      embed_github_org_name: embedding,
      mappings,
      output,
      snapshot,
    } = self;

    let mappings: Mappings = {
      let mut buf = Vec::new();
      let mut file = File::open(mappings)?;
      file.read_to_end(&mut buf)?;
      serde_yml::from_slice(&buf)?
    };

    let snapshot: Snapshot = {
      let mut buf = Vec::new();
      let mut file = File::open(snapshot)?;
      file.read_to_end(&mut buf)?;
      serde_yml::from_slice(&buf)?
    };

    let mut failed = false;

    for OktaGroupExpectation { id, profile_name } in &mappings.expectations.okta_groups {
      eprintln!("validating expectations for okta group with id: {id} ...");
      match snapshot.groups.iter().find(|group| group.id == id.as_str()) {
        None => {
          eprintln!("expected to be present in snapshot: FAILED");
          failed = true;
        }
        Some(group) => {
          if !(group.profile.name == profile_name.as_str()) {
            eprintln!(
              "expected profile name: '{profile_name}', but found '{}' instead",
              &group.profile.name
            );
            failed = true;
          }
        }
      }
    }

    if failed {
      std::process::exit(1);
    }

    let mut org_memberships = Vec::with_capacity(snapshot.users.len());

    for user in snapshot.users {
      if !user.status.is_active() {
        eprintln!("Skipping {} User: {}", &user.status, &user.profile.email);
        continue;
      }

      if let Some(ref usernames) = user.profile.github_username {
        for username in usernames {
          let username = username.trim().to_lowercase();
          if let Some(ref orgs) = user.profile.github_orgs {
            for org in orgs {
              if let Some(org) = mappings
                .orgs
                .iter()
                .find(|mapping| mapping.okta_profile_github_org.eq(org))
              {
                let org_name = &org.github_org_name;
                let role = mappings
                  .roles
                  .iter()
                  .find(|mapping| mapping.okta_profile_email.eq(&user.profile.email))
                  .map(|role| role.github_member_role)
                  .unwrap_or_default();

                let labels = {
                  let mut labels = HashMap::new();
                  labels.insert(LABEL_OKTA_USER_ID.into(), user.id.to_owned());
                  labels
                };

                let mut metadata = kubernetes::ObjectMeta {
                  name: username.clone(),
                  namespace: None,
                  annotations: None,
                  labels: Some(labels),
                };

                embedding.embed(org_name.to_owned(), &mut metadata);

                let resource = kubernetes::Resource {
                  api_version: String::from(github::Membership::API_GROUP_VERSION),
                  kind: String::from(github::Membership::KIND),
                  metadata,
                  spec: crossplane::ManagedResource {
                    deletion_policy: Some(crossplane::DeletionPolicy::Delete),
                    provider_config_ref: crossplane::ProviderConfigReference {
                      name: org_name.to_owned(),
                      policy: None,
                    },
                    for_provider: github::Membership {
                      downgrade_on_destroy: false,
                      role,
                      username: username.to_owned(),
                    },
                    init_provider: None,
                    management_policies: None,
                    publish_connection_details_to: None,
                    write_connection_secret_to_ref: None,
                  },
                };

                org_memberships.push(resource);
              }
            }
          }
        }
      }
    }

    let mut org_teams = HashSet::with_capacity(mappings.teams.len());

    let mut team_memberships = Vec::with_capacity(snapshot.group_users.values().map(Vec::len).sum());

    for (group, users) in snapshot.group_users {
      if let Some(team) = mappings.teams.iter().find(|mapping| mapping.okta_group_id.eq(&group)) {
        for user in users {
          if let Some(ref usernames) = user.profile.github_username {
            for username in usernames {
              if let Some(ref orgs) = user.profile.github_orgs {
                for org in orgs {
                  if let Some(org) = mappings
                    .orgs
                    .iter()
                    .find(|mapping| mapping.okta_profile_github_org.eq(org))
                  {
                    let team_name = team.github_team_name.as_str();
                    let org_name = org.github_org_name.as_str();

                    let labels = {
                      let mut labels = HashMap::new();
                      labels.insert(LABEL_OKTA_GROUP_ID.into(), group.clone());
                      labels.insert(LABEL_OKTA_USER_ID.into(), user.id.clone());
                      labels
                    };

                    let resource = kubernetes::Resource {
                      api_version: github::TeamMembership::API_GROUP_VERSION.into(),
                      kind: github::TeamMembership::KIND.into(),
                      metadata: kubernetes::ObjectMeta {
                        name: format!("{org_name}--{team_name}--{username}"),
                        namespace: None,
                        annotations: None,
                        labels: Some(labels),
                      },
                      spec: crossplane::ManagedResource {
                        for_provider: github::TeamMembership {
                          role: github::team_membership::Role::Member,
                          team_id: team_name.to_owned().into(),
                          username: username.to_owned(),
                        },
                        provider_config_ref: crossplane::ProviderConfigReference {
                          name: org_name.to_owned(),
                          policy: None,
                        },
                        deletion_policy: Some(crossplane::DeletionPolicy::Delete),
                        init_provider: None,
                        management_policies: None,
                        publish_connection_details_to: None,
                        write_connection_secret_to_ref: None,
                      },
                    };

                    org_teams.insert((org_name.to_owned(), (team_name.to_owned(), group.clone())));
                    team_memberships.push(resource);
                  }
                }
              }
            }
          }
        }
      }
    }

    let mut teams = Vec::with_capacity(org_teams.len());

    for (org_name, (team_name, group)) in org_teams {
      let labels = {
        let mut labels = HashMap::new();
        labels.insert(LABEL_OKTA_GROUP_ID.into(), group);
        labels
      };

      let resource = kubernetes::Resource {
        api_version: github::Team::API_GROUP_VERSION.into(),
        kind: github::Team::KIND.into(),
        metadata: kubernetes::ObjectMeta {
          name: format!("{org_name}--{team_name}"),
          namespace: None,
          annotations: None,
          labels: Some(labels),
        },
        spec: crossplane::ManagedResource {
          deletion_policy: Some(crossplane::DeletionPolicy::Delete),
          for_provider: github::Team {
            create_default_maintainer: false,
            description: None,
            ldap_dn: None,
            name: team_name,
            parent_team_id: None,
            privacy: github::team::Privacy::Closed,
          },
          provider_config_ref: ProviderConfigReference {
            name: org_name,
            policy: None,
          },
          init_provider: None,
          management_policies: None,
          publish_connection_details_to: None,
          write_connection_secret_to_ref: None,
        },
      };

      teams.push(resource);
    }

    let mut resources = Vec::with_capacity(org_memberships.len() + team_memberships.len() + teams.len());

    for resource in org_memberships {
      resources.push(serde_json::to_value(&resource)?);
    }

    for resource in teams {
      resources.push(serde_json::to_value(&resource)?);
    }

    for resource in team_memberships {
      resources.push(serde_json::to_value(&resource)?);
    }

    let yaml = serde_yml::to_string(&resources)?;

    fs_err::write(&output, &yaml)?;

    eprintln!("\n[OK] resources saved to {output}");

    Ok(())
  }
}
