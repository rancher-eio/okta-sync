use ::std::ops::RangeInclusive;
use proptest::option::OptionStrategy;
use proptest::prelude::*;

// FIXME - this does not accomodate longer email addresses properly
const EMAIL_PATTERN: &str = r"(?:[[:alnum:]][[:alnum:].-]+[[:alnum:]])@(?:[[:alnum:]][[:alnum:]-]{0,8}[[:alnum:]])(?:(?:[[:alnum:]][[:alnum:]-]{0,8}[[:alnum:]])){0,2}(?:[.][[:alpha:]]{2,8})";
const URL_PATTERN: &str = r"https?://[a-zA-Z0-9.+]{3,}+(?:[.][a-zA-Z]{2,9})(:[1-9][0-9]{1,3})?(/[a-zA-Z0-9]{0,9})?";

pub(crate) fn email() -> impl Strategy<Value = String> {
  prop::string::string_regex(EMAIL_PATTERN).expect("invalid regex")
}

pub(crate) mod email {
  use super::*;
  pub(crate) mod length {
    use super::*;
    pub(crate) fn within(range: RangeInclusive<usize>) -> impl Strategy<Value = String> {
      let max = *range.end();
      email()
        .prop_map(string::chars::take(max)) // FIXME - this can and will truncate domains and leave trailing dashes
        .prop_filter("invalid length", string::chars::count::within(range))
    }

    pub(crate) mod within {
      use super::*;

      pub(crate) fn option(range: RangeInclusive<usize>) -> impl Strategy<Value = Option<String>> {
        prop::option::of(within(range))
      }
    }
  }
}

pub(crate) fn url() -> impl Strategy<Value = String> {
  prop::string::string_regex(URL_PATTERN).expect("invalid regex")
}

pub(crate) mod url {
  use super::*;

  pub(crate) fn option() -> impl Strategy<Value = Option<String>> {
    prop::option::of(url())
  }
}

use std::string;

pub(crate) mod std {
  use super::*;

  pub(crate) fn string_with_length(range: RangeInclusive<usize>) -> BoxedStrategy<String> {
    let max = *(range.end());
    any::<String>()
      .prop_map(self::string::chars::take(max))
      .prop_filter("invalid length", self::string::chars::count::within(range))
      .boxed()
  }

  pub(crate) fn option_string_with_length(range: RangeInclusive<usize>) -> OptionStrategy<BoxedStrategy<String>> {
    prop::option::of(string_with_length(range))
  }

  pub(crate) mod string {
    use super::*;

    pub(crate) mod chars {
      use super::*;

      pub(crate) fn take(max: usize) -> impl Fn(String) -> String {
        move |string| string.chars().take(max).collect()
      }

      pub(crate) mod count {
        use super::*;

        pub(crate) fn within(range: RangeInclusive<usize>) -> impl Fn(&String) -> bool {
          move |string| range.contains(&string.chars().count())
        }
      }
    }
  }
}

pub(crate) mod chrono {
  use super::*;
  use ::chrono::{DateTime, Utc};

  pub(crate) fn datetime() -> BoxedStrategy<DateTime<Utc>> {
    any::<i64>().prop_map(DateTime::<Utc>::from_timestamp_nanos).boxed()
  }

  pub(crate) fn option_datetime() -> BoxedStrategy<Option<DateTime<Utc>>> {
    any::<Option<i64>>().prop_map(|n| n.map(DateTime::<Utc>::from_timestamp_nanos)).boxed()
  }
}
