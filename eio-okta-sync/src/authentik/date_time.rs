use std::str::FromStr;
use std::sync::LazyLock;

use chrono::{DateTime, FixedOffset, Local, Utc};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize)]
pub(crate) struct HumanFriendlyDateTime(DateTime<Utc>);

impl From<DateTime<Utc>> for HumanFriendlyDateTime {
  fn from(value: DateTime<Utc>) -> Self {
    Self(value)
  }
}

impl From<HumanFriendlyDateTime> for DateTime<Utc> {
  fn from(HumanFriendlyDateTime(value): HumanFriendlyDateTime) -> Self {
    value
  }
}

impl From<HumanFriendlyDateTime> for DateTime<FixedOffset> {
  fn from(value: HumanFriendlyDateTime) -> Self {
    DateTime::<Utc>::from(value).into()
  }
}

const MURICA: [&str; 3] = ["USA", "🇺🇸", "🦅"];

static DIALECT: LazyLock<interim::Dialect> = LazyLock::new(|| {
  if std::env::var("MURICA").is_ok_and(|freedom| MURICA.contains(&freedom.as_str())) {
    interim::Dialect::Us
  } else {
    interim::Dialect::Uk
  }
});

static NOW: LazyLock<DateTime<Utc>> = LazyLock::new(|| Local::now().to_utc());

impl FromStr for HumanFriendlyDateTime {
  type Err = interim::DateError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let parsed = match DateTime::parse_from_rfc3339(s) {
      Ok(parsed) => parsed.to_utc(),
      Err(_error) => interim::parse_date_string(s, *NOW, *DIALECT)?,
    };

    Ok(parsed.into())
  }
}

impl<'de> serde::Deserialize<'de> for HumanFriendlyDateTime {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>,
  {
    deserializer.deserialize_str(DateTimeVisitor)
  }
}

struct DateTimeVisitor;

impl<'de> serde::de::Visitor<'de> for DateTimeVisitor {
  type Value = HumanFriendlyDateTime;

  fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
    formatter.write_str("a valid date-time string")
  }

  fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
  where
    E: serde::de::Error,
  {
    v.parse().map_err(E::custom)
  }
}
