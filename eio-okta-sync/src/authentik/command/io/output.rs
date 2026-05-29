mod format;
mod target;

pub(crate) use format::*;
use serde::Serialize;
pub(crate) use target::*;

#[derive(Debug, Clone, PartialEq, Eq, Default, clap::Args, bon::Builder)]
#[group(id = "io.output")]
#[remain::sorted]
pub(crate) struct Output {
  #[arg(default_value = <&'static str>::from(OutputFormat::default()))]
  #[arg(help_heading = "I/O Options")]
  #[arg(id = "io.output.format")]
  #[arg(long = "output-format")]
  #[arg(value_name = "FORMAT")]
  #[builder(default)]
  pub(crate) format: OutputFormat,
  #[arg(default_value = AsRef::<str>::as_ref(DEFAULT_OUTPUT_TARGET_REF))]
  #[arg(help_heading = "I/O Options")]
  #[arg(id = "io.output.target")]
  #[arg(long = "output-target")]
  #[arg(value_name = "PATH or '-'")]
  #[builder(default)]
  pub(crate) target: OutputTarget,
}

#[derive(Debug, thiserror::Error, miette::Diagnostic)]
#[diagnostic(transparent)]
#[error(transparent)]
pub enum OutputError {
  Format(#[from] OutputFormatError),
  Target(#[from] OutputTargetError),
}

impl Output {
  pub(crate) fn write(self, value: &impl Serialize) -> Result<(), OutputError> {
    let Self { format, target } = self;

    let buf = format.stringify(&value)?;
    target.write(buf.as_bytes())?;

    Ok(())
  }
}

const DEFAULT_OUTPUT_TARGET: OutputTarget = OutputTarget::const_default();
const DEFAULT_OUTPUT_TARGET_REF: &OutputTarget = &DEFAULT_OUTPUT_TARGET;

#[cfg(test)]
mod tests {
  use super::*;

  crate::authentik::macros::test!(Default for struct Output);
  crate::authentik::macros::test!(Default == Builder for struct Output);
}
