/// convenience for rolling up errors, intended to facilitate a pattern of:
/// - any fallible function should return an error covering each way it can fail.
/// - those errors should be able to be easily generalized when you don't care about granularity.
macro_rules! From {
  ($FROM:ty => $ENUM:ident :: $VARIANT:ident) => {
    impl From<$FROM> for $ENUM {
      fn from(value: $FROM) -> Self {
        Self::$VARIANT(value)
      }
    }
  };

  ($SOURCE:ty => $VIA:ty => $TARGET:ty) => {
    const _: () = {
      use ::core::convert::From;

      impl From<$SOURCE> for $TARGET {
        fn from(value: $SOURCE) -> Self {
          <$VIA as From<$SOURCE>>::from(value).into()
        }
      }
    };
  };

  ($T0:ty => $T1:ty => $T2:ty => $T3:ty) => {
    crate::authentik::macros::From!($T0 => $T1 => $T2);
    crate::authentik::macros::From!($T0 => $T2 => $T3);
  };
  ($T0:ty => $T1:ty => $T2:ty => $T3:ty => $T4:ty) => {
    crate::authentik::macros::From!($T0 => $T1 => $T2);
    crate::authentik::macros::From!($T0 => $T2 => $T3);
    crate::authentik::macros::From!($T0 => $T3 => $T4);
  };
  ($T0:ty => $T1:ty => $T2:ty => $T3:ty => $T4:ty => $T5:ty) => {
    crate::authentik::macros::From!($T0 => $T1 => $T2);
    crate::authentik::macros::From!($T0 => $T2 => $T3);
    crate::authentik::macros::From!($T0 => $T3 => $T4);
    crate::authentik::macros::From!($T0 => $T4 => $T5);
  };
}

pub(crate) use From;

macro_rules! Paginated {
  ($PAGINATED:ty => $RESULT:ty) => {
    const _: () = {
      use ::authentik_client::models::Pagination;

      impl $crate::authentik::traits::Paginated for $PAGINATED {
        type Results = $RESULT;

        fn pagination(&self) -> &Pagination {
          &self.pagination
        }

        fn results(self) -> Self::Results {
          self.results
        }
      }
    };
  };
}

pub(crate) use Paginated;

macro_rules! PageMut {
  ($T:ty) => {
    const _: () = {
      use crate::authentik::traits::PageMut;

      impl PageMut for $T {
        fn page_mut(&mut self) -> Option<&mut i32> {
          self.page.as_mut()
        }
      }
    };
  };
}

pub(crate) use PageMut;

macro_rules! Default {
  ($T:ty as Builder) => {
    const _: () = {
      use std::default::Default;

      impl Default for $T {
        fn default() -> Self {
          Self::builder().build()
        }
      }
    };
  };

  (const fn for $T:ty as $VARIANT:ident) => {
    const _: () = {
      use std::default::Default;

      impl $T {
        pub(crate) const fn const_default() -> Self {
          Self::$VARIANT
        }
      }

      impl Default for $T {
        fn default() -> Self {
          Self::const_default()
        }
      }
    };
  };
}

pub(crate) use Default;

macro_rules! defaults {
  ($VIS:vis const $IDENT:ident : $TYPE:ty = $VALUE:literal as $STR:literal) => {
    ::paste::paste!(
      $VIS const [< $IDENT:upper >]: $TYPE = $VALUE;
      $VIS const [< $IDENT:upper _STR>]: &str = $STR;
      $VIS const fn [< $IDENT:lower >]() -> $TYPE {
        $IDENT
      }
    );
  };
}

pub(crate) use defaults;

#[cfg(test)]
macro_rules! test {
  (defaults for const $IDENT:ident as $TYPE:ty) => {
    ::paste::paste!(
      #[test]
      fn [< $IDENT:lower _parse>]() {
        let value = $IDENT;
        let parsed: $TYPE = [< $IDENT _STR>].parse().unwrap();

        assert_eq!(value, parsed);
      }

      #[test]
      fn [< $IDENT:lower _fn >]() {
        let value = $IDENT;
        let computed = [< $IDENT:lower >]();

        assert_eq!(value, computed);
      }
    );

  };

  (Default for struct $IDENT:ident) => {
    ::paste::paste!(
      #[test]
      fn [< $IDENT:snake:lower _default>]() {
        <$IDENT as Default>::default();
      }
    );
  };

  (Default == Deserialize for struct $IDENT:ident) => {
    ::paste::paste!(
      #[test]
      fn [< $IDENT:snake:lower _default_eq_deserialize>]() {
        let expected = $IDENT::default();
        let scenario: $IDENT = serde_json::from_str("{}").unwrap();

        assert_eq!(expected, scenario);
      }
    );
  };

  (Default == Builder for struct $IDENT:ident) => {
    ::paste::paste!(
      #[test]
      fn [< $IDENT:snake:lower _default_eq_builder>]() {
        let expected = $IDENT::default();
        let scenario = $IDENT::builder().build();

        assert_eq!(expected, scenario);
      }
    );
  };

  (Default == Parser for struct $IDENT:ident) => {
    ::paste::paste!(
      #[test]
      fn [< $IDENT:snake:lower _default_eq_parser>]() {
      use ::clap::Parser;

      let args: [&str; _] = [];

      let expected = $IDENT::default();
      let scenario = $IDENT::parse_from(args);

      assert_eq!(expected, scenario);
      }
    );
  };

  (defaults for struct $IDENT:ident) => {
    crate::authentik::macros::test!(Default for struct $IDENT);
    crate::authentik::macros::test!(Default == Deserialize for struct $IDENT);
    crate::authentik::macros::test!(Default == Builder for struct $IDENT);
    crate::authentik::macros::test!(Default == Parser for struct $IDENT);
  };
}

#[cfg(test)]
pub(crate) use test;

macro_rules! RunAsync {
  ($ENUM:ty as [ $($VARIANT:ident),+ ]) => {
    const _: () = {
      use crate::authentik::traits::RunAsync;

      impl RunAsync for $ENUM {
        async fn run(self) -> Result<(), crate::authentik::Error> {
          match self {
            $(
              Self::$VARIANT(inner) => inner.run().await?,
            )+
          }

          Ok(())
        }
      }
    };
  };

  ($T:ty as Wrapper<GetWithConfiguration>) => {
    const _: () = {
      use crate::authentik::command::{Wrapper,io::IoOptions};
      use crate::authentik::traits::{RunAsync,GetWithConfiguration};

      impl RunAsync for Wrapper<$T> {
        async fn run(self) -> Result<(), crate::authentik::Error> {
          let Self {
            authentik,
            autopaginate: _,
            dump,
            io: IoOptions { input, output },
            query,
          } = self;

          let query = input.read::<$T>()?.unwrap_or(query);

          if dump {
            output.write(&query)?;
          } else {
            let configuration = authentik.into();

            output.write(&query.get_with_configuration(&configuration).await?)?;
          }

          Ok(())
        }
      }
    };
  };
}

pub(crate) use RunAsync;
