use std::{io::Write, str::FromStr};

use camino::Utf8PathBuf;

#[derive(Debug, Clone, PartialEq, Eq, strum::AsRefStr)]
#[remain::sorted]
pub(crate) enum OutputTarget {
  #[strum(transparent)]
  Path(Utf8PathBuf),
  #[strum(serialize = "-")]
  Stdout,
}

#[derive(Debug, thiserror::Error, miette::Diagnostic)]
#[error(transparent)]
pub enum OutputTargetError {
  #[diagnostic(code(io::output::target::flush))]
  Flush(std::io::Error),
  #[diagnostic(code(io::output::target::open))]
  Open(std::io::Error),
  #[diagnostic(code(io::output::target::write))]
  Write(std::io::Error),
}

impl OutputTarget {
  fn open(self) -> Result<Box<dyn Write>, OutputTargetError> {
    match self {
      Self::Path(path) => match fs_err::File::create(&path) {
        Err(error) => Err(OutputTargetError::Open(error)),
        Ok(file) => Ok(Box::new(file)),
      },
      Self::Stdout => Ok(Box::new(std::io::stdout())),
    }
  }

  pub(crate) fn write(self, buf: &[u8]) -> Result<(), OutputTargetError> {
    let mut writer = self.open()?;
    writer.write_all(buf).map_err(OutputTargetError::Write)?;
    writer.flush().map_err(OutputTargetError::Flush)
  }
}

impl FromStr for OutputTarget {
  type Err = std::convert::Infallible;

  fn from_str(target: &str) -> Result<Self, Self::Err> {
    let target = if target == "-" {
      Self::Stdout
    } else {
      Utf8PathBuf::from_str(target)?.into()
    };

    Ok(target)
  }
}

crate::authentik::macros::Default!(const fn for OutputTarget as Stdout);
crate::authentik::macros::From!(Utf8PathBuf => OutputTarget::Path);

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn target_stdout() {
    let expected = Ok(OutputTarget::Stdout);
    let scenario = OutputTarget::from_str("-");
    assert_eq!(expected, scenario);
  }

  #[test]
  fn target_path() {
    let path = "/dev/null";
    let expected = Ok(OutputTarget::Path(Utf8PathBuf::from(path)));
    let scenario = OutputTarget::from_str(path);

    assert_eq!(expected, scenario);
  }
}
