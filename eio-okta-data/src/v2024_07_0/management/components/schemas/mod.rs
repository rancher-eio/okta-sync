pub mod authentication_provider;
pub mod authentication_provider_type;
pub mod digest_algorithm;
pub mod group;
pub mod group_profile;
pub mod group_type;
pub mod href_hints;
pub mod href_object;
pub mod href_object_self_link;
pub mod http_method;
pub mod links_self;
pub mod password_credential;
pub mod password_credential_hash;
pub mod password_credential_hash_algorithm;
pub mod password_credential_hook;
pub mod recovery_question_credential;
pub mod user;
pub mod user_credentials;
pub mod user_profile;
pub mod user_status;
pub mod user_type;

pub use authentication_provider::AuthenticationProvider;
pub use authentication_provider_type::AuthenticationProviderType;
pub use digest_algorithm::DigestAlgorithm;
pub use group::Group;
pub use group_profile::GroupProfile;
pub use group_type::GroupType;
pub use href_hints::HrefHints;
pub use href_object::HrefObject;
pub use href_object_self_link::HrefObjectSelfLink;
pub use http_method::HttpMethod;
pub use links_self::LinksSelf;
pub use password_credential::PasswordCredential;
pub use password_credential_hash::PasswordCredentialHash;
pub use password_credential_hash_algorithm::PasswordCredentialHashAlgorithm;
pub use password_credential_hook::PasswordCredentialHook;
pub use recovery_question_credential::RecoveryQuestionCredential;
pub use user::User;
pub use user_credentials::UserCredentials;
pub use user_profile::UserProfile;
pub use user_status::UserStatus;
pub use user_type::UserType;

#[crate::apply(crate::enums!)]
#[derive(Copy)]
#[cfg_attr(feature = "serde", serde(rename_all = "SCREAMING_SNAKE_CASE"))]
#[remain::sorted]
pub enum UserStatusTransitioningTo {
  Active,
  Deprovisioned,
  Provisioned,
}
