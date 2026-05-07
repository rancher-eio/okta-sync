pub(crate) mod cancel_enterprise_member_invitation;
pub(crate) mod invite_enterprise_member;
pub(crate) mod remove_enterprise_member;

use crate::graphql::*;

Builder!(Variables in cancel_enterprise_member_invitation::mutation);
Builder!(Variables in invite_enterprise_member::mutation);
Builder!(Variables in remove_enterprise_member::mutation);
