use crate::{crossplane, github, kubernetes};
use camino::Utf8PathBuf;
use clap::Parser;
use fs_err::File;
use iri_string::template::UriTemplateString;
use iri_string::types::UriAbsoluteString;
use serde::{Deserialize, Serialize};
use std::io::Read;

#[derive(Debug, Parser)]
#[remain::sorted]
#[rustfmt::skip]
#[command(about = "generate resources from current github users")]
#[group(skip)]
pub struct Command {
  #[arg(
    long,
    value_name    = "PATH",
    default_value = "members.yaml",
    help          = "file to read current members from",
  )]
  members: Utf8PathBuf,
  #[arg(
    long,
    help = "GitHub org to work with",
  )]
  org: String,
  #[arg(
    long,
    value_name    = "PATH",
    default_value = "current.yaml",
    help          = "file to write output to"
  )]
  output: Utf8PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[remain::sorted]
struct OrgMembership {
  avatar_url: UriAbsoluteString,
  events_url: UriTemplateString,
  followers_url: UriAbsoluteString,
  following_url: UriTemplateString,
  gists_url: UriTemplateString,
  gravatar_id: String,
  html_url: UriAbsoluteString,
  id: u64,
  pub login: String,
  node_id: String,
  organizations_url: UriAbsoluteString,
  received_events_url: UriAbsoluteString,
  repos_url: UriAbsoluteString,
  site_admin: bool,
  starred_url: UriTemplateString,
  subscriptions_url: UriAbsoluteString,
  #[serde(rename = "type")]
  type_: MemberType,
  url: UriAbsoluteString,
}

impl TryFrom<octocrab::models::Author> for OrgMembership {
  type Error = crate::Error;
  fn try_from(value: octocrab::models::Author) -> Result<Self, Self::Error> {
    let octocrab::models::Author {
      avatar_url,
      email: _,
      events_url,
      followers_url,
      following_url,
      gists_url,
      gravatar_id,
      html_url,
      id,
      login,
      node_id,
      organizations_url,
      patch_url: _,
      r#type: type_,
      received_events_url,
      repos_url,
      site_admin,
      starred_url,
      subscriptions_url,
      url,
      ..
    } = value;

    let this = Self {
      avatar_url: avatar_url.as_str().try_into()?,
      events_url: events_url.as_str().try_into()?,
      followers_url: followers_url.as_str().try_into()?,
      following_url: following_url.as_str().try_into()?,
      gists_url: gists_url.as_str().try_into()?,
      gravatar_id,
      html_url: html_url.as_str().try_into()?,
      id: id.into_inner(),
      login,
      node_id,
      organizations_url: organizations_url.as_str().try_into()?,
      received_events_url: received_events_url.as_str().try_into()?,
      repos_url: repos_url.as_str().try_into()?,
      site_admin,
      starred_url: starred_url.as_str().try_into()?,
      subscriptions_url: subscriptions_url.as_str().try_into()?,
      type_: serde_yml::from_str(&type_)?,
      url: url.as_str().try_into()?,
    };

    Ok(this)
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[remain::sorted]
enum MemberType {
  #[default]
  User,
}

impl Command {
  #[remain::check]
  pub fn run(self) -> Result<(), crate::Error> {
    let Self { members, org, output } = self;

    let members: Vec<OrgMembership> = {
      let mut buf = Vec::new();
      let mut file = File::open(&members)?;
      file.read_to_end(&mut buf)?;
      serde_yml::from_slice(&buf)?
    };

    let mut resources = Vec::with_capacity(members.len());
    for member in members {
      let username = member.login.to_lowercase();
      let resource = kubernetes::Resource {
        api_version: String::from(github::Membership::API_GROUP_VERSION),
        kind: String::from(github::Membership::KIND),
        metadata: kubernetes::ObjectMeta {
          name: format!("{org}--{username}"),
          namespace: None,
          annotations: None,
          labels: None,
        },
        spec: crossplane::ManagedResource {
          deletion_policy: Some(crossplane::DeletionPolicy::Delete),
          provider_config_ref: crossplane::ProviderConfigReference {
            name: org.to_owned(),
            policy: None,
          },
          for_provider: github::Membership {
            downgrade_on_destroy: false,
            role: github::membership::Role::Member,
            username: username.to_owned(),
          },
          init_provider: None,
          management_policies: None,
          publish_connection_details_to: None,
          write_connection_secret_to_ref: None,
        },
      };

      resources.push(resource);
    }

    let yaml = serde_yml::to_string(&resources)?;

    fs_err::write(&output, &yaml)?;

    eprintln!("\n[OK] resources saved to {output}");

    Ok(())
  }
}
