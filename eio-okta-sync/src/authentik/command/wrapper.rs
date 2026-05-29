use clap::{Args, Parser};
use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::authentik::command::io::IoOptions;
use crate::authentik::options::AuthentikOptions;
use crate::authentik::traits::{AutopaginateWithConfiguration, GetWithConfiguration, Paginated, RunAsync};

#[derive(Debug, Clone, Parser)]
pub(crate) struct Wrapper<T>
where
  T: Args,
{
  #[clap(flatten)]
  pub(crate) authentik: AuthentikOptions,
  #[arg(action = clap::ArgAction::Set)]
  #[arg(default_value = "true")]
  #[arg(global = true)]
  #[arg(long)]
  #[arg(value_name = "BOOL")]
  pub(crate) autopaginate: bool,
  #[arg(action = clap::ArgAction::Set)]
  #[arg(default_missing_value = "true")]
  #[arg(default_value = "false")]
  #[arg(global = true)]
  #[arg(long)]
  #[arg(value_name = "BOOL")]
  pub(crate) dump: bool,
  #[command(flatten)]
  pub(crate) io: IoOptions,
  #[clap(flatten)]
  pub(crate) query: T,
}

impl<T> RunAsync for Wrapper<T>
where
  T: Args + GetWithConfiguration + AutopaginateWithConfiguration + Serialize + DeserializeOwned,
  T::Value: Paginated + Serialize,
  <T::Value as Paginated>::Results: Serialize,
  crate::authentik::Error: From<T::Error>,
{
  async fn run(self) -> Result<(), crate::authentik::Error> {
    let Self {
      authentik,
      autopaginate,
      dump,
      io: IoOptions { input, output },
      query,
    } = self;

    let query = input.read::<T>()?.unwrap_or(query);

    if dump {
      output.write(&query)?;
    } else {
      let configuration = authentik.into();

      if autopaginate {
        output.write(&query.autopaginate_with_configuration(&configuration).await?)?;
      } else {
        output.write(&query.get_with_configuration(&configuration).await?)?;
      };
    }

    Ok(())
  }
}
