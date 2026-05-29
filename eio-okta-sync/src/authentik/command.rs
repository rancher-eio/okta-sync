pub(crate) mod io;

mod wrapper;

pub(crate) use wrapper::Wrapper;

#[derive(Debug, Clone, clap::Subcommand)]
pub(crate) enum Command {
  #[command(subcommand)]
  Api(api::Command),
}

crate::authentik::macros::RunAsync!(Command as [Api]);

pub(crate) mod api {
  #[derive(Debug, Clone, clap::Subcommand)]
  #[command(about = "Authentik REST API")]
  #[remain::sorted]
  pub(crate) enum Command {
    #[command(subcommand)]
    Core(core::Command),
  }

  crate::authentik::macros::RunAsync!(Command as [Core]);

  pub(crate) mod core {
    #[allow(clippy::large_enum_variant)]
    #[derive(Debug, Clone, clap::Subcommand)]
    #[command(about = "Core APIs")]
    #[remain::sorted]
    pub(crate) enum Command {
      #[command(subcommand)]
      Groups(groups::Command),
      #[command(subcommand)]
      Tokens(tokens::Command),
      #[command(subcommand)]
      Users(users::Command),
    }

    crate::authentik::macros::RunAsync!(Command as [ Groups, Tokens, Users ]);

    pub(crate) mod groups {
      use crate::authentik::{api, command::Wrapper};

      #[derive(Debug, Clone, clap::Subcommand)]
      #[command(about = "Group Operations")]
      #[remain::sorted]
      pub(crate) enum Command {
        List(Box<Wrapper<api::CoreGroupsList>>),
        #[command(alias = "get")]
        Retrieve(Box<Wrapper<api::CoreGroupsRetrieve>>),
      }

      crate::authentik::macros::RunAsync!(Command as [List, Retrieve]);
    }

    pub(crate) mod tokens {
      use crate::authentik::{api, command::Wrapper};

      #[derive(Debug, Clone, clap::Subcommand)]
      #[command(about = "Token Operations")]
      #[remain::sorted]
      pub(crate) enum Command {
        List(Box<Wrapper<api::CoreTokensList>>),
        #[command(alias = "get")]
        Retrieve(Box<Wrapper<api::CoreTokensRetrieve>>),
      }

      crate::authentik::macros::RunAsync!(Command as [List, Retrieve]);
    }

    pub(crate) mod users {
      use crate::authentik::{api, command::Wrapper};

      #[derive(Debug, Clone, clap::Subcommand)]
      #[command(about = "User Operations")]
      #[remain::sorted]
      pub(crate) enum Command {
        List(Box<Wrapper<api::CoreUsersList>>),
        Me(Box<Wrapper<api::CoreUsersMeRetrieve>>),
        #[command(alias = "get")]
        Retrieve(Box<Wrapper<api::CoreUsersRetrieve>>),
      }

      crate::authentik::macros::RunAsync!(Command as [List, Me, Retrieve]);
    }
  }
}
