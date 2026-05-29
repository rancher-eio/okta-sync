use serde::de::DeserializeOwned;

mod format;
mod source;

pub(crate) use format::*;
pub(crate) use source::*;

#[derive(Debug, Clone, PartialEq, Eq, clap::Args, bon::Builder)]
#[group(id = "io.input")]
#[remain::sorted]
pub(crate) struct Input {
  #[arg(default_value = <&'static str>::from(InputFormat::default()))]
  #[arg(help_heading = "I/O Options")]
  #[arg(id = "io.input.format")]
  #[arg(long = "input-format")]
  #[arg(value_name = "FORMAT")]
  #[builder(default)]
  pub(crate) format: InputFormat,
  #[arg(help_heading = "I/O Options")]
  #[arg(id = "io.input.source")]
  #[arg(long = "input-source")]
  #[arg(value_name = "PATH or '-'")]
  pub(crate) source: Option<InputSource>,
}

#[derive(Debug, thiserror::Error, miette::Diagnostic)]
#[error(transparent)]
pub enum InputError {
  Deserialize(#[from] erased_serde::Error),
  Format(#[from] InputFormatError),
  Source(#[from] InputSourceError),
}

impl Input {
  pub(crate) fn read<T: DeserializeOwned>(self) -> Result<Option<T>, InputError> {
    let Self { format, source } = self;

    match source {
      None => Ok(None),
      Some(source) => {
        let bytes = source.bytes()?;
        let mut deserializer = format.deserializer(&bytes)?;
        let value = T::deserialize(&mut *deserializer)?;
        Ok(Some(value))
      }
    }
  }
}
