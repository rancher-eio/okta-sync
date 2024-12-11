#[cfg(feature = "clap")]
pub(crate) mod clap {
  use std::{ffi::OsString, iter::empty};

  pub(crate) use ::clap::{Args, Parser};

  #[derive(Debug, Parser)]
  pub(crate) struct FlatCommand<T: Args> {
    #[command(flatten)]
    pub(crate) args: T,
  }

  impl<T: Args> FlatCommand<T> {
    pub(crate) fn from_empty() -> Self {
      Self::parse_from(empty::<OsString>())
    }
  }
}

macro_rules! defaults_match {
  ($t:ty) => {
    #[cfg(feature = "clap")]
    #[test]
    fn clap_defaults_match() {
      let expected = <$t>::default();
      let actual = crate::testing::clap::FlatCommand::<$t>::from_empty().args;
      assert_eq!(expected, actual)
    }
  };
}

pub(crate) use defaults_match;

macro_rules! validate {
  ($t:ty) => {
    #[cfg(all(feature = "proptest", feature = "validate"))]
    #[::test_strategy::proptest]
    fn validate_arbitrary(this: $t) {
      assert_eq!(Ok(()), ::validator::Validate::validate(&this))
    }

    #[cfg(all(feature = "dummy", feature = "validate"))]
    #[test]
    fn validate_dummy() {
      use ::fake::Fake;

      let this = ::fake::Faker.fake::<$t>();
      assert_eq!(Ok(()), ::validator::Validate::validate(&this))
    }
  };
}

pub(crate) use validate;

macro_rules! roundtrip {
  ($t:ty) => {
    #[cfg(all(feature = "proptest", feature = "serde"))]
    #[::test_strategy::proptest]
    fn roundtrip_arbitrary(this: $t) {
      let json = serde_json::to_string(&this).expect("serialization failure");
      let that = serde_json::from_str(&json).expect("deserialization failure");
      assert_eq!(this, that);
    }
  };
}

pub(crate) use roundtrip;
