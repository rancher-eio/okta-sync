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
        Create(Box<Wrapper<api::CoreGroupsCreate>>),
        Destroy(Box<Wrapper<api::CoreGroupsDestroy>>),
        List(Box<Wrapper<api::CoreGroupsList>>),
        ListUsedBy(Box<Wrapper<api::CoreGroupsUsedByList>>),
        #[command(alias = "get")]
        Retrieve(Box<Wrapper<api::CoreGroupsRetrieve>>),
        #[command(subcommand)]
        Update(update::Command),
        #[command(subcommand)]
        User(user::Command),
      }

      crate::authentik::macros::RunAsync!(Command as [Create, Destroy, List, ListUsedBy, Retrieve, Update, User]);

      pub(crate) mod update {
        use crate::authentik::{api, command::Wrapper};

        #[derive(Debug, Clone, clap::Subcommand)]
        #[command(about = "Group Update Operations")]
        #[remain::sorted]
        pub(crate) enum Command {
          #[command(alias = "replace")]
          Overwrite(Box<Wrapper<api::CoreGroupsUpdate>>),
          Patch(Box<Wrapper<api::CoreGroupsPartialUpdate>>),
        }

        crate::authentik::macros::RunAsync!(Command as [Overwrite, Patch]);
      }

      pub(crate) mod user {
        use crate::authentik::{api, command::Wrapper};

        #[derive(Debug, Clone, clap::Subcommand)]
        #[command(about = "Group User Operations")]
        #[remain::sorted]
        pub(crate) enum Command {
          Add(Box<Wrapper<api::CoreGroupsAddUserCreate>>),
          Remove(Box<Wrapper<api::CoreGroupsRemoveUserCreate>>),
        }

        crate::authentik::macros::RunAsync!(Command as [Add, Remove]);
      }
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
