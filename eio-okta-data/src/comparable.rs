use std::collections::BTreeMap;

use ::chrono::{Datelike, Timelike};
use ::mediatype::ReadParams;

#[derive(Debug, Clone, PartialEq, Eq, Hash, ::comparable::Comparable)]
pub struct MediaType {
  type_: String,
  sub_type: String,
  suffix: Option<String>,
  params: BTreeMap<String, String>,
}

impl From<::mediatype::MediaType<'_>> for MediaType {
  fn from(borrowed: ::mediatype::MediaType) -> Self {
    let type_ = borrowed.ty.as_str().to_owned();
    let sub_type = borrowed.subty.as_str().to_owned();
    let suffix = borrowed.suffix.map(|suffix| suffix.as_str().to_owned());
    let params = borrowed
      .params()
      .map(|(name, value)| (name.as_str().to_owned(), value.as_str().to_owned()))
      .collect();

    Self {
      type_,
      sub_type,
      suffix,
      params,
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, ::comparable::Comparable)]
pub struct DateTime {
  year: i32,
  month: u32,
  day: u32,
  hour: u32,
  minute: u32,
  second: u32,
}

impl From<::chrono::DateTime<::chrono::Utc>> for DateTime {
  fn from(datetime: ::chrono::DateTime<::chrono::Utc>) -> Self {
    let naive = datetime.naive_utc();

    let date = naive.date();
    let year = date.year();
    let month = date.month();
    let day = date.day();

    let time = naive.time();
    let hour = time.hour();
    let minute = time.minute();
    let second = time.second();

    Self {
      year,
      month,
      day,
      hour,
      minute,
      second,
    }
  }
}
