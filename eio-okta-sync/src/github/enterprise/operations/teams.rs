use octocrab::Octocrab;

mod create;
mod delete;
mod get;
mod list;
mod memberships;
mod organizations;
mod update;

#[derive(Debug, bon::Builder)]
pub struct EnterpriseTeamsHandler<'client, 'enterprise> {
  client: &'client Octocrab,
  enterprise: &'enterprise str,
}

impl super::EnterpriseHandler<'_, '_> {
  pub fn teams(&self) -> EnterpriseTeamsHandler<'_, '_> {
    EnterpriseTeamsHandler::builder()
      .client(self.client)
      .enterprise(self.enterprise)
      .build()
  }
}

#[derive(Debug, bon::Builder)]
pub struct EnterpriseTeamHandler<'client, 'enterprise, 'team> {
  client: &'client Octocrab,
  enterprise: &'enterprise str,
  team: &'team str,
}

impl super::EnterpriseHandler<'_, '_> {
  pub fn team<'team>(&self, team: &'team str) -> EnterpriseTeamHandler<'_, '_, 'team> {
    EnterpriseTeamHandler::builder()
      .client(self.client)
      .enterprise(self.enterprise)
      .team(team)
      .build()
  }
}
