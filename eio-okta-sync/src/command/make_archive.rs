use std::io::{Cursor, Write};

use camino::Utf8PathBuf;
use serde_yml::Value;
use tar::{Builder, Header};

use crate::{
  crossplane::ManagedResource,
  github::{Membership, Team, TeamMembership},
  kubernetes::Resource,
};

use super::generate::EmbedGithubOrgName;

#[derive(Debug, Clone, clap::Args)]
#[remain::sorted]
#[rustfmt::skip]
#[command(about = "create an archive from generated resources")]
pub struct Command {
  #[command(flatten)]
  embed_github_org_name: EmbedGithubOrgName,
  #[arg(
    long,
    value_name    = "BOOL",
    default_value = "false",
    action        = clap::ArgAction::Set,
    help_heading = "Output Options",
    help         = "append YAML end-of-document marker ('...')",
  )]
  force_yaml_end_of_document: bool,
  #[arg(
    long,
    value_name    = "BOOL",
    default_value = "true",
    action        = clap::ArgAction::Set,
    help_heading = "Output Options",
    help         = "append YAML start-of-document marker ('---')",
  )]
  force_yaml_start_of_document: bool,
  #[arg(
    long,
    value_name    = "KIND",
    default_value = Membership::KIND,
    help_heading = "Input Options",
    help = "which kind of resources to collect in the archive",
  )]
  kind: Kind,
  #[arg(
    long,
    value_name = "ORG",
    help      = "GitHub Org to read from",
    long_help = "GitHub Org to read from.

This is assumed to be a prefix of the $.metadata.name for each resource, and will be stripped from the filename.",
  )]
  org: String,
  #[arg(
    long,
    value_name    = "PATH",
    default_value = "resources.tar",
    help_heading = "Output Options",
    help         = "file to write output to (as a tarball)",
  )]
  output: Utf8PathBuf,
  #[arg(
    long,
    value_name    = "PATH",
    default_value = "resources.yaml",
    help_heading = "Input Options",
    help         = "file to read resources from",
  )]
  resources: Utf8PathBuf,
  #[arg(
    long,
    value_name    = "BOOL",
    default_value = "true",
    action = clap::ArgAction::Set,
    help_heading = "Output Options",
    help         = "strip namespaces from resources",
  )]
  strip_namespaces: bool,
}

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
enum Kind {
  #[value(name = Membership::KIND)]
  Membership,
  #[value(name = Team::KIND)]
  Team,
  #[value(name = TeamMembership::KIND)]
  TeamMembership,
}

impl AsRef<str> for Kind {
  fn as_ref(&self) -> &str {
    match self {
      Self::Membership => Membership::KIND,
      Self::Team => Team::KIND,
      Self::TeamMembership => TeamMembership::KIND,
    }
  }
}

/// u+rw,g+r,o+r
const DEFAULT_MODE: u32 = 0o644;
const YAML_END_OF_DOCUMENT: &str = "...\n";
const YAML_START_OF_DOCUMENT: &str = "---\n";

impl Command {
  pub fn run(self) -> Result<(), crate::Error> {
    let Self {
      embed_github_org_name: embedded,
      kind: target,
      org,
      output: output_path,
      resources: resources_path,
      strip_namespaces,
      force_yaml_end_of_document,
      force_yaml_start_of_document,
    } = self;

    let resources_yaml = fs_err::read_to_string(&resources_path)?;
    let mut resources: Vec<Resource<ManagedResource<Value>>> = serde_yml::from_str(&resources_yaml)?;

    let mut archive = Builder::new(Cursor::new(Vec::new()));

    for resource in resources.iter_mut() {
      let Resource { kind, metadata, .. } = resource;
      if kind.as_str().eq(target.as_ref()) {
        if let Some(org_name) = embedded.org(metadata) {
          if org == org_name {
            if strip_namespaces {
              metadata.namespace = None;
            }
            let name = embedded.name(metadata);
            let path = Utf8PathBuf::from(format!("manifests/resources/{kind}/{name}.yaml"));
            let mut header = Header::new_gnu();
            header.set_mode(DEFAULT_MODE);
            let mut entry = archive.append_writer(&mut header, path)?;
            if force_yaml_start_of_document {
              entry.write_all(YAML_START_OF_DOCUMENT.as_bytes())?;
            }
            serde_yml::to_writer(&mut entry, &resource)?;
            if force_yaml_end_of_document {
              entry.write_all(YAML_END_OF_DOCUMENT.as_bytes())?;
            }
            entry.finish()?;
          }
        }
      }
    }

    archive.finish()?;

    let contents = archive.into_inner()?.into_inner();

    fs_err::write(&output_path, &contents)?;

    eprintln!("\n[OK] resources saved to {output_path}");

    Ok(())
  }
}
