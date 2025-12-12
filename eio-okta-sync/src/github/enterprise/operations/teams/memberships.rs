use octocrab::Octocrab;

mod add;
mod bulk_add;
mod bulk_remove;
mod get;
mod list;
mod remove;

#[derive(Debug, bon::Builder)]
pub struct EnterpriseTeamMembershipsHandler<'client, 'enterprise, 'team> {
  client: &'client Octocrab,
  enterprise: &'enterprise str,
  team: &'team str,
}

impl super::EnterpriseTeamHandler<'_, '_, '_> {
  pub fn memberships(&self) -> EnterpriseTeamMembershipsHandler<'_, '_, '_> {
    EnterpriseTeamMembershipsHandler::builder()
      .client(self.client)
      .enterprise(self.enterprise)
      .team(self.team)
      .build()
  }
}

#[derive(Debug, bon::Builder)]
pub struct EnterpriseTeamMembershipHandler<'client, 'enterprise, 'team, 'username> {
  client: &'client Octocrab,
  enterprise: &'enterprise str,
  team: &'team str,
  username: &'username str,
}

impl super::EnterpriseTeamHandler<'_, '_, '_> {
  pub fn membership<'membership>(
    &self,
    username: &'membership str,
  ) -> EnterpriseTeamMembershipHandler<'_, '_, '_, 'membership> {
    EnterpriseTeamMembershipHandler::builder()
      .client(self.client)
      .enterprise(self.enterprise)
      .team(self.team)
      .username(username)
      .build()
  }
}
