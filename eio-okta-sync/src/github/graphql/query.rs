pub(crate) mod enterprise;
pub(crate) mod enterprise_member_invitation;
pub(crate) mod enterprise_members;
pub(crate) mod enterprise_organizations;
pub(crate) mod enterprise_pending_admin_invitations;
pub(crate) mod enterprise_pending_collaborator_invitations;
pub(crate) mod enterprise_pending_member_invitations;
pub(crate) mod enterprise_pending_unaffiliated_member_invitations;
pub(crate) mod organization;
pub(crate) mod user;
pub(crate) mod viewer_enterprises;

use crate::graphql::*;

Builder!(Variables in enterprise::query);
Builder!(Variables in enterprise_member_invitation::query);
Builder!(Variables in enterprise_members::query);
Builder!(Variables in enterprise_organizations::query);
Builder!(Variables in enterprise_pending_admin_invitations::query);
Builder!(Variables in enterprise_pending_collaborator_invitations::query);
Builder!(Variables in enterprise_pending_member_invitations::query);
Builder!(Variables in enterprise_pending_unaffiliated_member_invitations::query);
Builder!(Variables in organization::query);
Builder!(Variables in user::query);
Builder!(Variables in viewer_enterprises::query);

const _: () = {
  use enterprise_members::{Query, query::*};

  impl PageInfo for ResponseData {
    type PageInfo = QueryEnterpriseMembersPageInfo;

    fn page_info(&self) -> Option<&Self::PageInfo> {
      Some(&self.enterprise.as_ref()?.members.page_info)
    }
  }

  ForwardPagination!(Query as GraphQLQuery);

  crate::macros::IntoIterator!(ResponseData => {
    (self) -> Vec<QueryEnterpriseMembersNodes> {
      { self.enterprise } => FilterMap [members.nodes].flatten().flatten()
    }
  });
};

const _: () = {
  use enterprise_organizations::{Query, query::*};

  impl PageInfo for ResponseData {
    type PageInfo = QueryEnterpriseOrganizationsPageInfo;

    fn page_info(&self) -> Option<&Self::PageInfo> {
      Some(&self.enterprise.as_ref()?.organizations.page_info)
    }
  }

  ForwardPagination!(Query as GraphQLQuery);

  crate::macros::IntoIterator!(ResponseData => {
    (self) -> Vec<QueryEnterpriseOrganizationsNodes> {
      { self.enterprise } => FilterMap [organizations.nodes].flatten().flatten()
    }
  });
};

const _: () = {
  use enterprise_pending_admin_invitations::{Query, query::*};

  impl PageInfo for ResponseData {
    type PageInfo = QueryEnterpriseOwnerInfoPendingAdminInvitationsPageInfo;

    fn page_info(&self) -> Option<&Self::PageInfo> {
      Some(
        &self
          .enterprise
          .as_ref()?
          .owner_info
          .as_ref()?
          .pending_admin_invitations
          .page_info,
      )
    }
  }

  ForwardPagination!(Query as GraphQLQuery);

  crate::macros::IntoIterator!(ResponseData => {
    (self) -> Vec<QueryEnterpriseOwnerInfoPendingAdminInvitationsNodes> {
      { self.enterprise } => FilterMap [owner_info, pending_admin_invitations.nodes].flatten().flatten()
    }
  });
};

const _: () = {
  use enterprise_pending_collaborator_invitations::{Query, query::*};

  impl PageInfo for ResponseData {
    type PageInfo = QueryEnterpriseOwnerInfoPendingCollaboratorInvitationsPageInfo;

    fn page_info(&self) -> Option<&Self::PageInfo> {
      Some(
        &self
          .enterprise
          .as_ref()?
          .owner_info
          .as_ref()?
          .pending_collaborator_invitations
          .page_info,
      )
    }
  }

  ForwardPagination!(Query as GraphQLQuery);

  crate::macros::IntoIterator!(ResponseData => {
    (self) -> Vec<QueryEnterpriseOwnerInfoPendingCollaboratorInvitationsNodes> {
      { self.enterprise } => FilterMap  [owner_info, pending_collaborator_invitations.nodes].flatten().flatten()
    }
  });
};

const _: () = {
  use enterprise_pending_member_invitations::{Query, query::*};

  impl PageInfo for ResponseData {
    type PageInfo = QueryEnterpriseOwnerInfoPendingMemberInvitationsPageInfo;

    fn page_info(&self) -> Option<&Self::PageInfo> {
      Some(
        &self
          .enterprise
          .as_ref()?
          .owner_info
          .as_ref()?
          .pending_member_invitations
          .page_info,
      )
    }
  }

  ForwardPagination!(Query as GraphQLQuery);

  crate::macros::IntoIterator!(ResponseData => {
    (self) -> Vec<QueryEnterpriseOwnerInfoPendingMemberInvitationsNodes> {
      { self.enterprise } => FilterMap [owner_info, pending_member_invitations.nodes].flatten().flatten()
    }
  });
};

const _: () = {
  use enterprise_pending_unaffiliated_member_invitations::{Query, query::*};

  impl PageInfo for ResponseData {
    type PageInfo = QueryEnterpriseOwnerInfoPendingUnaffiliatedMemberInvitationsPageInfo;

    fn page_info(&self) -> Option<&Self::PageInfo> {
      Some(
        &self
          .enterprise
          .as_ref()?
          .owner_info
          .as_ref()?
          .pending_unaffiliated_member_invitations
          .page_info,
      )
    }
  }

  ForwardPagination!(Query as GraphQLQuery);

  crate::macros::IntoIterator!(ResponseData => {
    (self) -> Vec<QueryEnterpriseOwnerInfoPendingUnaffiliatedMemberInvitationsNodes> {
      { self.enterprise } => FilterMap [owner_info, pending_unaffiliated_member_invitations.nodes].flatten().flatten()
    }
  });
};

const _: () = {
  use viewer_enterprises::{Query, query::*};

  impl PageInfo for ResponseData {
    type PageInfo = QueryViewerEnterprisesPageInfo;

    fn page_info(&self) -> Option<&Self::PageInfo> {
      Some(&self.viewer.enterprises.as_ref()?.page_info)
    }
  }

  ForwardPagination!(Query as GraphQLQuery);

  crate::macros::IntoIterator!(ResponseData => {
    (self) -> Vec<QueryViewerEnterprisesNodes> {
      { self.viewer.enterprises } => FilterMap [nodes].flatten().flatten()
    }
  });
};
