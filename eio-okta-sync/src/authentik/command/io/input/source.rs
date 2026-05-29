use std::{io::Read, str::FromStr};

use camino::Utf8PathBuf;

#[derive(Debug, Clone, PartialEq, Eq, strum::AsRefStr)]
#[remain::sorted]
pub(crate) enum InputSource {
  #[strum(transparent)]
  Path(Utf8PathBuf),
  #[strum(serialize = "-")]
  Stdin,
}

#[derive(Debug, thiserror::Error, miette::Diagnostic)]
#[error(transparent)]
pub enum InputSourceError {
  #[diagnostic(code(io::input::source::open))]
  Open(std::io::Error),
  #[diagnostic(code(io::input::source::read))]
  Read(std::io::Error),
}

impl InputSource {
  fn reader(self) -> Result<Box<dyn std::io::Read>, InputSourceError> {
    match self {
      Self::Path(path) => match fs_err::File::open(&path) {
        Err(error) => Err(InputSourceError::Open(error)),
        Ok(file) => Ok(Box::new(file)),
      },
      Self::Stdin => Ok(Box::new(std::io::stdin())),
    }
  }

  pub(crate) fn bytes(self) -> Result<Vec<u8>, InputSourceError> {
    let mut buf = Vec::new();
    self.reader()?.read_to_end(&mut buf).map_err(InputSourceError::Read)?;
    Ok(buf)
  }
}

impl FromStr for InputSource {
  type Err = std::convert::Infallible;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    if s == "-" {
      Ok(Self::Stdin)
    } else {
      Ok(Self::Path(Utf8PathBuf::from_str(s)?))
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn stdin() {
    let expected = Ok(InputSource::Stdin);
    let scenario = InputSource::from_str("-");
    assert_eq!(expected, scenario);
  }

  #[test]
  fn path() {
    let path = "/dev/null";
    let expected = Ok(InputSource::Path(Utf8PathBuf::from(path)));
    let scenario = InputSource::from_str(path);

    assert_eq!(expected, scenario);
  }
}
