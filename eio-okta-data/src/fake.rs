macro_rules! fake {
  ($locale:ident, $name:ident) => {
    #[allow(dead_code)]
    #[allow(non_snake_case)]
    pub(crate) fn $name() -> super::raw::$name<$crate::fake::locales::$locale> {
      super::raw::$name($crate::fake::locales::$locale)
    }
  };
  ($name:ident) => {
    fake!(Custom, $name);
  };
}

macro_rules! fake_address {
  ($module:ident => $locale:ident) => {
    pub(crate) mod $module {
      fake!($locale, BuildingNumber);
      fake!($locale, CityName);
      fake!($locale, CityPrefix);
      fake!($locale, CitySuffix);
      fake!($locale, CountryCode);
      fake!($locale, CountryName);
      fake!($locale, Latitude);
      fake!($locale, Longitude);
      fake!($locale, PostCode);
      fake!($locale, PostalAddress);
      fake!($locale, SecondaryAddress);
      fake!($locale, SecondaryAddressType);
      fake!($locale, StateAbbr);
      fake!($locale, StateName);
      fake!($locale, StreetAddress);
      fake!($locale, StreetSuffix);
      fake!($locale, TimeZone);
      fake!($locale, ZipCode);
    }
  };
}

macro_rules! fake_company {
  ($module:ident => $locale:ident) => {
    pub(crate) mod $module {
      fake!($locale, Bs);
      fake!($locale, BsAdj);
      fake!($locale, BsNoun);
      fake!($locale, BsVerb);
      fake!($locale, Buzzword);
      fake!($locale, BuzzwordMiddle);
      fake!($locale, BuzzwordTail);
      fake!($locale, CatchPhrase);
      fake!($locale, CompanyName);
      fake!($locale, CompanySuffix);
      fake!($locale, Industry);
      fake!($locale, Profession);
    }
  };
}

macro_rules! fake_internet {
  ($module:ident => $locale:ident) => {
    pub(crate) mod $module {
      fake!($locale, DomainSuffix);
      fake!($locale, FreeEmail);
      fake!($locale, FreeEmailProvider);
      fake!($locale, IP);
      fake!($locale, IPv4);
      fake!($locale, IPv6);
      fake!($locale, MACAddress);
      fake!($locale, SafeEmail);
      fake!($locale, UserAgent);
      fake!($locale, Username);
    }
  };
}

macro_rules! fake_job {
  ($module:ident => $locale:ident) => {
    pub(crate) mod $module {
      fake!($locale, Field);
      fake!($locale, Position);
      fake!($locale, Seniority);
      fake!($locale, Title);
    }
  };
}

macro_rules! fake_name {
  ($module:ident => $locale:ident) => {
    pub(crate) mod $module {
      fake!($locale, FirstName);
      fake!($locale, LastName);
      fake!($locale, Name);
      fake!($locale, NameWithTitle);
      fake!($locale, Suffix);
      fake!($locale, Title);
    }
  };
}

macro_rules! fake_phone_number {
  ($module:ident => $locale:ident) => {
    pub(crate) mod $module {
      fake!($locale, CellNumber);
      fake!($locale, PhoneNumber);
    }
  };
}

pub(crate) mod address {
  fake_address!(en => EN);
  fake_address!(custom => Custom);

  mod raw {
    pub(crate) use ::fake::faker::address::raw::*;
    use ::fake::{locales::Data, Dummy, Fake};
    use ::rand::Rng;

    pub(crate) struct StreetAddress<L>(pub L);

    impl<L> StreetAddress<L> {
      const TEMPLATE: &'static str = "{BuildingNumber} {StreetName}";
    }

    impl<L: Data + Copy> Dummy<StreetAddress<L>> for String {
      fn dummy_with_rng<R: Rng + ?Sized>(c: &StreetAddress<L>, rng: &mut R) -> Self {
        StreetAddress::<L>::TEMPLATE
          .replace("{BuildingNumber}", BuildingNumber(c.0).fake_with_rng::<String, _>(rng).as_str())
          .replace("{StreetName}", StreetName(c.0).fake_with_rng::<String, _>(rng).as_str())
      }
    }

    pub(crate) struct PostalAddress<L>(pub L);

    impl<L> PostalAddress<L> {
      const TEMPLATE: &'static str = "{StreetAddress} {CityName} {StateName} {PostCode} {CountryName}";
    }

    impl<L: Data + Copy> Dummy<PostalAddress<L>> for String {
      fn dummy_with_rng<R: Rng + ?Sized>(c: &PostalAddress<L>, rng: &mut R) -> Self {
        PostalAddress::<L>::TEMPLATE
          .replace("{StreetAddress}", StreetAddress(c.0).fake_with_rng::<String, _>(rng).as_str())
          .replace("{CityName}", CityName(c.0).fake_with_rng::<String, _>(rng).as_str())
          .replace("{StateName}", StateName(c.0).fake_with_rng::<String, _>(rng).as_str())
          .replace("{PostCode}", PostCode(c.0).fake_with_rng::<String, _>(rng).as_str())
          .replace("{CountryName}", CountryName(c.0).fake_with_rng::<String, _>(rng).as_str())
      }
    }
  }
}

pub(crate) mod company {
  fake_company!(en => EN);
  fake_company!(custom => Custom);

  pub(crate) mod raw {
    pub(crate) use ::fake::faker::company::raw::*;
  }
}

pub(crate) mod internet {
  fake_internet!(en => EN);
  fake_internet!(custom => Custom);

  pub(crate) mod raw {
    pub(crate) use ::fake::faker::internet::raw::*;
  }
}

pub(crate) mod job {
  fake_job!(en => EN);
  fake_job!(custom => Custom);

  pub(crate) mod raw {
    pub(crate) use ::fake::faker::job::raw::*;
  }
}

pub(crate) mod locale {
  pub(crate) mod custom {
    fake!(Language);
  }

  pub(crate) mod en {
    use crate::fake::locales::EN;

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    pub(crate) fn Language() -> super::raw::Language<EN> {
      super::raw::Language(EN)
    }
  }

  pub(crate) mod raw {
    use ::fake::{locales::Data, Dummy, Fake};
    use ::rand::{seq::SliceRandom, Rng};

    pub struct Language<L>(pub L);

    impl<L> Language<L> {
      // since any list of languages will be incomplete, this const is named
      // this way in order to make the selection criteria obvious.
      //
      // Source: https://docs.rs/lingua/1.6.2/lingua/enum.Language.html
      const DETECTABLE_BY_LINGUA_V1_6_2: &'static [&'static str] = &[
        "Afrikaans",
        "Albanian",
        "Arabic",
        "Armenian",
        "Azerbaijani",
        "Basque",
        "Belarusian",
        "Bengali",
        "Bokmal",
        "Bosnian",
        "Bulgarian",
        "Catalan",
        "Chinese",
        "Croatian",
        "Czech",
        "Danish",
        "Dutch",
        "English",
        "Esperanto",
        "Estonian",
        "Finnish",
        "French",
        "Ganda",
        "Georgian",
        "German",
        "Greek",
        "Gujarati",
        "Hebrew",
        "Hindi",
        "Hungarian",
        "Icelandic",
        "Indonesian",
        "Irish",
        "Italian",
        "Japanese",
        "Kazakh",
        "Korean",
        "Latin",
        "Latvian",
        "Lithuanian",
        "Macedonian",
        "Malay",
        "Maori",
        "Marathi",
        "Mongolian",
        "Nynorsk",
        "Persian",
        "Polish",
        "Portuguese",
        "Punjabi",
        "Romanian",
        "Russian",
        "Serbian",
        "Shona",
        "Slovak",
        "Slovene",
        "Somali",
        "Sotho",
        "Spanish",
        "Swahili",
        "Swedish",
        "Tagalog",
        "Tamil",
        "Telugu",
        "Thai",
        "Tsonga",
        "Tswana",
        "Turkish",
        "Ukrainian",
        "Urdu",
        "Vietnamese",
        "Welsh",
        "Xhosa",
        "Yoruba",
        "Zulu",
      ];
    }

    impl<L: Data> Dummy<Language<L>> for &str {
      fn dummy_with_rng<R: Rng + ?Sized>(_: &Language<L>, rng: &mut R) -> Self {
        Language::<L>::DETECTABLE_BY_LINGUA_V1_6_2.choose(rng).unwrap()
      }
    }

    impl<L: Data> Dummy<Language<L>> for String {
      fn dummy_with_rng<R: Rng + ?Sized>(l: &Language<L>, rng: &mut R) -> Self {
        l.fake_with_rng::<&str, _>(rng).to_owned()
      }
    }
  }
}

pub(crate) mod name {
  fake_name!(en => EN);
  fake_name!(custom => Custom);

  pub(crate) mod raw {
    pub(crate) use ::fake::faker::name::raw::*;
  }
}

pub(crate) mod phone_number {
  fake_phone_number!(en => EN);
  fake_phone_number!(custom => Custom);

  pub(crate) mod raw {
    pub(crate) use ::fake::faker::phone_number::raw::*;
  }
}

pub(crate) mod locales {
  use ::constcat::concat_slices;
  use ::fake::locales::Data;
  pub(crate) use ::fake::locales::{AR_SA, FR_FR, JA_JP, PT_BR, ZH_CN, ZH_TW};

  #[derive(Clone, Copy)]
  pub(crate) struct EN;
  impl Data for EN {
    const ADDRESS_CITY_TPL: &'static str = "{CityName}{CitySuffix}";
    const ADDRESS_CITY_WITH_PREFIX_TPL: &'static str = "{CityPrefix} {CityName}{CitySuffix}";
  }

  #[derive(Clone, Copy)]
  pub(crate) struct Custom;

  impl Data for Custom {
    const ADDRESS_CITY_TPL: &'static str = "{CityName}{CitySuffix}";
    const ADDRESS_CITY_WITH_PREFIX_TPL: &'static str = "{CityPrefix} {CityName}{CitySuffix}";
    const ADDRESS_POSTCODE_FORMATS: &'static [&'static str] = concat_slices!([&'static str]: EN::ADDRESS_POSTCODE_FORMATS, PT_BR::ADDRESS_POSTCODE_FORMATS);
    const ADDRESS_STATE: &'static [&'static str] = concat_slices!([&'static str]: EN::ADDRESS_STATE, PT_BR::ADDRESS_STATE);
    const ADDRESS_STATE_ABBR: &'static [&'static str] = concat_slices!([&'static str]: EN::ADDRESS_STATE_ABBR, PT_BR::ADDRESS_STATE_ABBR);
    const ADDRESS_STREET_SUFFIX: &'static [&'static str] = concat_slices!([&'static str]: EN::ADDRESS_STREET_SUFFIX, PT_BR::ADDRESS_STREET_SUFFIX);
    const COMPANY_NAME_TPLS: &'static [&'static str] = concat_slices!([&'static str]: EN::COMPANY_NAME_TPLS, JA_JP::COMPANY_NAME_TPLS);
    const COMPANY_SUFFIX: &'static [&'static str] = concat_slices!([&'static str]: EN::COMPANY_SUFFIX, JA_JP::COMPANY_SUFFIX, PT_BR::COMPANY_SUFFIX);
    const INTERNET_DOMAIN_SUFFIX: &'static [&'static str] = concat_slices!([&'static str]: EN::INTERNET_DOMAIN_SUFFIX, PT_BR::INTERNET_DOMAIN_SUFFIX);
    const INTERNET_FREE_EMAIL_PROVIDER: &'static [&'static str] =
      concat_slices!([&'static str]: EN::INTERNET_FREE_EMAIL_PROVIDER, FR_FR::INTERNET_FREE_EMAIL_PROVIDER, PT_BR::INTERNET_FREE_EMAIL_PROVIDER);
    const JOB_FIELD: &'static [&'static str] = concat_slices!([&'static str]: EN::JOB_FIELD, JA_JP::JOB_FIELD, ZH_CN::JOB_FIELD);
    const JOB_POSITION: &'static [&'static str] = concat_slices!([&'static str]: EN::JOB_POSITION, JA_JP::JOB_POSITION, ZH_CN::JOB_POSITION);
    const JOB_SENIORITY: &'static [&'static str] = concat_slices!([&'static str]: EN::JOB_SENIORITY, JA_JP::JOB_SENIORITY, ZH_CN::JOB_SENIORITY);
    const NAME_FIRST_NAME: &'static [&'static str] = concat_slices!([&'static str]: AR_SA::NAME_FIRST_NAME, EN::NAME_FIRST_NAME, FR_FR::NAME_FIRST_NAME, JA_JP::NAME_FIRST_NAME, PT_BR::NAME_FIRST_NAME, ZH_CN::NAME_FIRST_NAME, ZH_TW::NAME_FIRST_NAME);
    const NAME_LAST_NAME: &'static [&'static str] = concat_slices!([&'static str]: AR_SA::NAME_LAST_NAME, EN::NAME_LAST_NAME, FR_FR::NAME_LAST_NAME, JA_JP::NAME_LAST_NAME, PT_BR::NAME_LAST_NAME, ZH_CN::NAME_LAST_NAME, ZH_TW::NAME_LAST_NAME);
    const NAME_TITLE: &'static [&'static str] = concat_slices!([&'static str]: EN::NAME_TITLE, JA_JP::NAME_TITLE, ZH_CN::NAME_TITLE, ZH_TW::NAME_TITLE);
    const PHONE_CELL_NUMBER_FORMATS: &'static [&'static str] = concat_slices!([&'static str]: EN::PHONE_CELL_NUMBER_FORMATS, FR_FR::PHONE_CELL_NUMBER_FORMATS, JA_JP::PHONE_CELL_NUMBER_FORMATS, PT_BR::PHONE_CELL_NUMBER_FORMATS);
    const PHONE_NUMBER_FORMATS: &'static [&'static str] =
      concat_slices!([&'static str]: EN::PHONE_NUMBER_FORMATS, FR_FR::PHONE_NUMBER_FORMATS, JA_JP::PHONE_NUMBER_FORMATS, PT_BR::PHONE_NUMBER_FORMATS);
  }
}
