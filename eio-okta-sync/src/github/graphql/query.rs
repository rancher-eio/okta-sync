pub(crate) mod enterprise;
pub(crate) mod enterprise_members;
pub(crate) mod enterprise_organizations;
pub(crate) mod enterprise_pending_member_invitations;
pub(crate) mod enterprise_pending_unaffiliated_member_invitations;
pub(crate) mod viewer_enterprises;

use crate::graphql::*;

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
