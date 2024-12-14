use std::{str::FromStr, time::Duration};

use camino::Utf8PathBuf;
use iri_string::types::{UriAbsoluteString, UriRelativeString};
use semver::Version;
use update_informer::{registry, Check};

#[derive(Debug, Clone, clap::Args)]
#[remain::sorted]
#[rustfmt::skip]
#[command(about = "check for updates to this program")]
pub struct Command {
  #[arg(
    env  = "CHECK_VERSION_OUTPUT_FILE",
    long = "output-file",
    value_name = "PATH",
    help_heading = "Output Options",
    help         = "file to write output to (defaults to STDOUT if omitted)",
  )]
  file: Option<Utf8PathBuf>,
  #[arg(
    env  = "CHECK_VERSION_OUTPUT_FORMAT",
    long = "output-format",
    value_name    = "FORMAT",
    default_value = "json",
    help_heading = "Output Options",
    help         = "output format",
  )]
  format: Format,
  #[arg(
    env  = "CHECK_VERSION_CACHE_INTERVAL",
    long = "cache-interval",
    value_name    = "DURATION",
    value_parser  = humantime::parse_duration,
    default_value = "0s",
    help_heading = "Cache Options",
    hide_short_help = true,
    long_help = "Sets an interval how often to check for a new version.

NOTE: This is set to 0s by default to DISABLE caching.

The first time each package is checked, the version given is cached
WITHOUT checking upstream. Only after the cache interval has passed will
the next check be done.

This means that when caching is enabled, the first check will ALWAYS
report that you have the latest version.

This is counterintuitive when used as one-shot command, but makes more
sense when running as a CronJob in a distributed environment such as
Kubernetes.

This is also why this option is hidden from the summary help.

You probably don't want to configure this unless you have a reason to.",
  )]
  interval: Duration,
  #[arg(
    env  = "CHECK_VERSION_PACKAGE_NAME",
    long = "package-name",
    value_name = "NAME",
    help_heading = "Package Options",
    help         = "what to check for updates (defaults to itself, see --help for details)",
    long_help    = constcat::concat!("check some other package, rather than this one.

the update checking system isn't specific to this package, and there's no reason
it CAN'T check other packages, so this is configurable for that reason (and testing).

the default value depends on what CHECK_VERSION_FROM_SOURCE is set to.
  - crates: ", env!("CARGO_PKG_NAME"), "
  - github: ", env!("CARGO_PKG_REPOSITORY"), "(only the path, without the leading slash)")
  )]
  name: Option<UriRelativeString>,
  #[arg(
    env  = "CHECK_VERSION_PACKAGE_REGISTRY",
    long = "package-registry",
    value_name    = "SOURCE",
    default_value = "crates-io",
    help_heading = "Package Options",
    help         = "where to check for updates",
  )]
  registry: Registry,
  #[arg(
    env  = "CHECK_VERSION_TIMEOUT",
    long = "timeout",
    value_name    = "DURATION",
    value_parser  = humantime::parse_duration,
    default_value = "5s",
    help_heading = "Network Options",
    help = "Sets a request timeout",
  )]
  timeout: Duration,
  #[arg(
    env  = "CHECK_VERSION_PACKAGE_VERSION",
    long = "package-version",
    value_name    = "SEMVER",
    default_value = env!("CARGO_PKG_VERSION"),
    help_heading = "Package Options",
    help         = "override the package version",
    long_help    = "override the package version.

By default, this is the current version of the program itself, but the same
mechanism can be used to check other packages from the same sources...",
  )]
  version: Version,
}

#[derive(Debug, Clone, Copy, Default, clap::ValueEnum)]
#[clap(rename_all = "kebab-case")]
pub enum Registry {
  #[default]
  CratesIO,
  Github,
}

#[derive(Debug, Clone, Copy, Default, clap::ValueEnum)]
pub enum Format {
  #[default]
  Json,
  Yaml,
}

impl Format {
  fn serialize<T: serde::Serialize>(self, value: &T) -> Result<String, crate::Error> {
    match self {
      Self::Json => serde_json::to_string_pretty(value).map_err(Into::into),
      Self::Yaml => serde_yml::to_string(value).map_err(Into::into),
    }
  }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct Package {
  name: String,
  versions: Versions,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct Versions {
  current: Version,
  latest: Version,
}

impl Command {
  pub fn run(self) -> Result<(), crate::Error> {
    let Self {
      file,
      format,
      name,
      interval,
      registry,
      timeout,
      version,
    } = self;

    let name: String = match name {
      Some(uri) => uri.path_str().trim_start_matches('/').into(),
      None => match registry {
        Registry::CratesIO => env!("CARGO_PKG_NAME").into(),
        Registry::Github => UriAbsoluteString::from_str(env!("CARGO_PKG_REPOSITORY"))?
          .path_str()
          .trim_start_matches('/')
          .into(),
      },
    };

    let package = match match registry {
      Registry::CratesIO => update_informer::new(registry::Crates, &name, &version.to_string())
        .interval(interval)
        .timeout(timeout)
        .check_version(),
      Registry::Github => update_informer::new(registry::GitHub, &name, &version.to_string())
        .interval(interval)
        .timeout(timeout)
        .check_version(),
    } {
      Ok(Some(latest)) => Package {
        name,
        versions: Versions {
          current: version,
          latest: latest.semver().to_owned(),
        },
      },
      Ok(None) => Package {
        name,
        versions: Versions {
          current: version.clone(),
          latest: version.clone(),
        },
      },
      Err(err) => {
        eprintln!("🤬 {err}");
        std::process::exit(100);
      }
    };

    let output = format.serialize(&package)?;

    match file {
      None => println!("{output}"),
      Some(path) => {
        fs_err::write(&path, output)?;
        eprintln!("[OK] output written to {path}");
      }
    }

    Ok(())
  }
}
