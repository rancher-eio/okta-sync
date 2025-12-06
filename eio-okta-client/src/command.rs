mod standalone;

use clap::{Parser, Subcommand};
use eio_okta_api::v1::{groups, users};

#[derive(Debug, Parser)]
#[group(skip)]
pub struct Command {
  #[command(flatten)]
  client: crate::Options,
  #[command(subcommand)]
  operation: Operation,
  #[arg(long)]
  pretty: bool,
}

#[derive(Debug, Subcommand)]
#[remain::sorted]
pub enum Operation {
  AssignUserToGroup(groups::AssignUserToGroup),
  GetGroup(groups::GetGroup),
  GetUser(users::GetUser),
  ListAssignedApplicationsForGroup(groups::ListAssignedApplicationsForGroup),
  ListGroups(groups::ListGroups),
  ListGroupUsers(groups::ListGroupUsers),
  ListUsers(users::ListUsers),
  UnassignUserFromGroup(groups::UnassignUserFromGroup),
}

impl Command {
  #[remain::check]
  pub fn run(self) -> Result<(), crate::Error> {
    let Command {
      client,
      operation,
      pretty,
    } = self;

    let client = crate::Client::from(client);

    let json = {
      #[remain::sorted]
      match operation {
        Operation::AssignUserToGroup(endpoint) => client.json(endpoint, pretty),
        Operation::GetGroup(endpoint) => client.json(endpoint, pretty),
        Operation::GetUser(endpoint) => client.json(endpoint, pretty),
        Operation::ListAssignedApplicationsForGroup(endpoint) => client.autopaginate_json(endpoint, pretty),
        Operation::ListGroups(endpoint) => client.autopaginate_json(endpoint, pretty),
        Operation::ListGroupUsers(endpoint) => client.autopaginate_json(endpoint, pretty),
        Operation::ListUsers(endpoint) => client.autopaginate_json(endpoint, pretty),
        Operation::UnassignUserFromGroup(endpoint) => client.json(endpoint, pretty),
      }
    }?;

    println!("{json}");

    Ok(())
  }
}

pub use standalone::Standalone;
