use octocrab::Octocrab;

mod add;
mod bulk_add;
mod bulk_remove;
mod get;
mod list;
mod remove;

#[derive(Debug, bon::Builder)]
pub struct EnterpriseTeamOrganizationsHandler<'client, 'enterprise, 'team> {
  client: &'client Octocrab,
  enterprise: &'enterprise str,
  team: &'team str,
}

impl super::EnterpriseTeamHandler<'_, '_, '_> {
  pub fn organizations(&self) -> EnterpriseTeamOrganizationsHandler<'_, '_, '_> {
    EnterpriseTeamOrganizationsHandler::builder()
      .client(self.client)
      .enterprise(self.enterprise)
      .team(self.team)
      .build()
  }
}

#[derive(Debug, bon::Builder)]
pub struct EnterpriseTeamOrganizationHandler<'client, 'enterprise, 'team, 'org> {
  client: &'client Octocrab,
  enterprise: &'enterprise str,
  team: &'team str,
  org: &'org str,
}

impl super::EnterpriseTeamHandler<'_, '_, '_> {
  pub fn organization<'org>(&self, org: &'org str) -> EnterpriseTeamOrganizationHandler<'_, '_, '_, 'org> {
    EnterpriseTeamOrganizationHandler::builder()
      .client(self.client)
      .enterprise(self.enterprise)
      .team(self.team)
      .org(org)
      .build()
  }
}
