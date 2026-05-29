use erased_serde::Deserializer;
use serde::Deserialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, clap::ValueEnum, strum::IntoStaticStr)]
#[clap(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
#[remain::sorted]
pub(crate) enum InputFormat {
  #[default]
  Json,
  Yaml,
}

#[derive(Debug, thiserror::Error, miette::Diagnostic)]
#[error(transparent)]
pub enum InputFormatError {
  #[diagnostic(code(io::input::format::json::value))]
  JsonValue(serde_json::Error),
  #[diagnostic(code(io::input::format::yaml::value))]
  YamlValue(serde_yml::Error),
}

impl InputFormat {
  pub(crate) fn deserializer<'de>(self, bytes: &'de [u8]) -> Result<Box<dyn Deserializer<'de>>, InputFormatError> {
    match self {
      Self::Json => {
        let mut deserializer = serde_json::Deserializer::from_slice(bytes);
        match serde_json::Value::deserialize(&mut deserializer) {
          Err(error) => Err(InputFormatError::JsonValue(error)),
          Ok(deserializer) => Ok(Box::new(<dyn Deserializer>::erase(deserializer))),
        }
      }
      Self::Yaml => {
        let deserializer = serde_yml::Deserializer::from_slice(bytes);
        match serde_yml::Value::deserialize(deserializer) {
          Err(error) => Err(InputFormatError::YamlValue(error)),
          Ok(deserializer) => Ok(Box::new(<dyn Deserializer>::erase(deserializer))),
        }
      }
    }
  }
}
