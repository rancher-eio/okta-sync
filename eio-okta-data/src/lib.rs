#![allow(non_camel_case_types)]

#[cfg(feature = "comparable")]
pub mod comparable;

pub mod v2024_07_0;
pub use v2024_07_0 as current;

macro_rules! impl_from {
  ($from:ty => $into:ty as $field:ident) => {
    impl From<$from> for $into {
      fn from(value: $from) -> Self {
        value.$field
      }
    }
  };
}

pub(crate) use impl_from;

macro_rules! impl_as_ref {
  ($self:ty) => {
    impl AsRef<$self> for $self {
      fn as_ref(&self) -> &$self {
        &self
      }
    }
  };
  ($self:ty => $referent:ty as $field:ident) => {
    impl AsRef<$referent> for $self {
      fn as_ref(&self) -> &$referent {
        self.$field.as_ref()
      }
    }
  };
}

pub(crate) use impl_as_ref;

#[macro_use]
pub(crate) extern crate macro_rules_attribute;
pub(crate) use macro_rules_attribute::apply;
pub(crate) use macro_rules_attribute::attribute_alias;

attribute_alias! {
  #[apply(structs)] =
    #[derive(crate::macro_rules_attribute::Custom)]
    #[cfg_attr(feature = "educe", derive(::educe::Educe))]
    #[cfg_attr(feature = "educe", educe(Debug, Clone, PartialEq, Eq, Hash))]
    #[cfg_attr(feature = "arbitrary",  derive(::arbitrary::Arbitrary))]
    #[cfg_attr(feature = "builder",    derive(::bon::Builder))]
    #[cfg_attr(feature = "builder",    builder(on(String, into)))]
    #[cfg_attr(feature = "comparable", derive(::comparable::Comparable))]
    #[cfg_attr(feature = "dummy",      derive(::fake::Dummy))]
    #[cfg_attr(feature = "proptest",   derive(::proptest_derive::Arbitrary))]
    #[cfg_attr(feature = "schemars",   derive(::schemars::JsonSchema))]
    #[cfg_attr(feature = "serde",      derive(::serde::Deserialize))]
    #[cfg_attr(feature = "serde",      derive(::serde::Serialize))]
    #[cfg_attr(feature = "patch",      derive(::struct_patch::Patch))]
    #[cfg_attr(feature = "validate",   derive(::validator::Validate))]
    #[cfg_attr(all(feature = "patch", feature = "educe"),      patch(attribute(derive(::educe::Educe))))]
    #[cfg_attr(all(feature = "patch", feature = "educe"),      patch(attribute(educe(Debug, Clone, PartialEq, Hash, Default))))]
    #[cfg_attr(all(feature = "patch", feature = "builder"),    patch(attribute(derive(::bon::Builder))))]
    #[cfg_attr(all(feature = "patch", feature = "builder"),    patch(attribute(builder(on(String, into)))))]
    #[cfg_attr(all(feature = "patch", feature = "schemars"),   patch(attribute(derive(::schemars::JsonSchema))))]
    #[cfg_attr(all(feature = "patch", feature = "serde"),      patch(attribute(derive(::serde::Deserialize))))]
    #[cfg_attr(all(feature = "patch", feature = "serde"),      patch(attribute(derive(::serde::Serialize))))]
    #[cfg_attr(all(feature = "patch", feature = "validate"),   patch(attribute(derive(::validator::Validate))))]
    #[cfg_attr(all(feature = "patch", feature = "comparable"), patch(attribute(derive(::comparable::Comparable))))]
  ;

  #[apply(structs_with_args)] =
    #[crate::apply(crate::structs)]
    #[cfg_attr(feature = "clap",         derive(::clap::Args))]
  ;

  // #[apply(structs_with_parser)] =
  //   #[crate::apply(crate::structs)]
  //   #[cfg_attr(feature = "clap",         derive(::clap::Parser))]
  // ;

  #[apply(enums)] =
    #[cfg_attr(feature = "educe",      derive(::educe::Educe))]
    #[cfg_attr(feature = "educe",      educe(Debug, Clone, PartialEq, Eq, Hash))]
    #[cfg_attr(feature = "arbitrary",  derive(::arbitrary::Arbitrary))]
    #[cfg_attr(feature = "clap",       derive(::clap::ValueEnum))]
    #[cfg_attr(feature = "comparable", derive(::comparable::Comparable))]
    #[cfg_attr(feature = "dummy",      derive(::fake::Dummy))]
    #[cfg_attr(feature = "proptest",   derive(::proptest_derive::Arbitrary))]
    #[cfg_attr(feature = "schemars",   derive(::schemars::JsonSchema))]
    #[cfg_attr(feature = "serde",      derive(::serde::Deserialize))]
    #[cfg_attr(feature = "serde",      derive(::serde::Serialize))]
    #[cfg_attr(feature = "strum",      derive(::strum::AsRefStr))]
    #[cfg_attr(feature = "strum",      derive(::strum::Display))]
    #[cfg_attr(feature = "strum",      derive(::strum::EnumCount))]
    #[cfg_attr(feature = "strum",      derive(::strum::EnumDiscriminants))]
    #[cfg_attr(feature = "strum",      derive(::strum::EnumIs))]
    #[cfg_attr(feature = "strum",      derive(::strum::EnumIter))]
    #[cfg_attr(feature = "strum",      derive(::strum::EnumMessage))]
    #[cfg_attr(feature = "strum",      derive(::strum::EnumProperty))]
    #[cfg_attr(feature = "strum",      derive(::strum::EnumString))]
    #[cfg_attr(feature = "strum",      derive(::strum::EnumTryAs))]
    #[cfg_attr(feature = "strum",      derive(::strum::FromRepr))]
    #[cfg_attr(feature = "strum",      derive(::strum::IntoStaticStr))]
    #[cfg_attr(feature = "strum",      derive(::strum::VariantArray))]
    #[cfg_attr(feature = "strum",      derive(::strum::VariantNames))]
  ;

  // #[apply(errors)] =
  //   #[cfg_attr(feature = "educe",      derive(::educe::Educe))]
  //   #[cfg_attr(feature = "educe",      educe(Debug))]
  //   #[cfg_attr(feature = "errors",     derive(::thiserror::Error))]
  //   #[cfg_attr(feature = "diagnostic", derive(::miette::Diagnostic))]
  // ;

}

#[cfg(feature = "dummy")]
pub(crate) mod fake;

#[cfg(feature = "proptest")]
pub(crate) mod strategy;

#[cfg(test)]
pub(crate) mod testing;
