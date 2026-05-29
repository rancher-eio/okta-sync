use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, clap::ValueEnum, strum::IntoStaticStr)]
#[clap(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
#[remain::sorted]
pub(crate) enum OutputFormat {
  CompactJson,
  #[default]
  Json,
  Yaml,
}

#[derive(Debug, thiserror::Error, miette::Diagnostic)]
#[error(transparent)]
pub enum OutputFormatError {
  #[diagnostic(code(io::output::format::json))]
  Json(serde_json::Error),
  #[diagnostic(code(io::output::format::yaml))]
  Yaml(serde_yml::Error),
}

impl OutputFormat {
  pub(crate) fn stringify<T: Serialize>(self, value: &T) -> Result<String, OutputFormatError> {
    match self {
      Self::CompactJson => serde_json::to_string(value).map_err(OutputFormatError::Json),
      Self::Json => serde_json::to_string_pretty(value).map_err(OutputFormatError::Json),
      Self::Yaml => serde_yml::to_string(value).map_err(OutputFormatError::Yaml),
    }
  }
}
