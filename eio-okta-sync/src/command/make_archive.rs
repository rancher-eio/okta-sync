use std::io::Cursor;

use camino::Utf8PathBuf;
use serde_yml::Value;
use tar::{Builder, Header};

use crate::{
  crossplane::ManagedResource,
  github::{Membership, Team, TeamMembership},
  kubernetes::{ObjectMeta, Resource},
};

#[derive(Debug, Clone, clap::Args)]
#[remain::sorted]
#[rustfmt::skip]
#[command(about = "create an archive from generated resources")]
pub struct Command {
  #[arg(
    long,
    value_name    = "KIND",
    default_value = Membership::KIND,
    help = "which kind of resources to collect in the archive",
  )]
  kind: Kind,
  #[arg(
    long,
    value_name = "ORG",
    help = "GitHub Org to read from",
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

impl Command {
  pub fn run(self) -> Result<(), crate::Error> {
    let Self {
      kind: target,
      org,
      output: output_path,
      resources: resources_path,
    } = self;

    let prefix = format!("{org}--");

    let resources_yaml = fs_err::read_to_string(&resources_path)?;
    let resources: Vec<Resource<ManagedResource<Value>>> = serde_yml::from_str(&resources_yaml)?;

    let mut archive = Builder::new(Cursor::new(Vec::new()));

    for resource in resources.iter() {
      let Resource {
        kind,
        metadata: ObjectMeta { name, .. },
        ..
      } = resource;
      if kind.eq(target.as_ref()) {
        if let Some(name) = name.strip_prefix(&prefix) {
          let path = Utf8PathBuf::from(format!("manifests/resources/{kind}/{name}.yaml"));
          let mut header = Header::new_gnu();
          let mut entry = archive.append_writer(&mut header, path)?;
          serde_yml::to_writer(&mut entry, &resource)?;
          entry.finish()?;
        }
      }
    }

    archive.finish()?;

    let contents = archive.into_inner()?.into_inner();

    fs_err::write(output_path, &contents)?;

    Ok(())
  }
}
