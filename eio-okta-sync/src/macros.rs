macro_rules! IntoIterator {
  (use Vec as Collection) => {
    use std::vec::IntoIter;
    use std::vec::Vec as Collection;
  };

  (use BTreeSet as Collection) => {
    use std::collections::btree_set::IntoIter;
    use std::collections::btree_set::BTreeSet as Collection;
  };

  ($T:ty => {
    ($SELF:ident) -> $COLLECTION:ident<&$ITEM:ty> { $EXPR:expr }
  }) => {
    const _: () = {
      crate::macros::IntoIterator!(use $COLLECTION as Collection);

      impl<'iter> IntoIterator for &'iter $T {
        type IntoIter = IntoIter<Self::Item>;
        type Item = &'iter $ITEM;

        fn into_iter($SELF) -> Self::IntoIter {
          { $EXPR }.collect::<Collection<_>>().into_iter()
        }
      }
    };
  };

  ($T:ty => {
    ($SELF:ident) -> $COLLECTION:ident<$ITEM:ty> { $EXPR:expr }
  }) => {
    const _: () = {
      crate::macros::IntoIterator!(use $COLLECTION as Collection);

      impl IntoIterator for $T {
        type IntoIter = IntoIter<Self::Item>;
        type Item = $ITEM;

        fn into_iter($SELF) -> Self::IntoIter {
          { $EXPR }.collect::<Collection<_>>().into_iter()
        }
      }
    };
  };

  ($T:ty => {
    ($SELF:ident) -> $COLLECTION:ident<$ITEM:ty> {
      { $($PRE:tt)+ } => where {
        AsRef => { $($ASREF:tt)+ },
        Owned => { $($OWNED:tt)+ },
      } $($POST:tt)+
    }
  }) => {
    crate::macros::IntoIterator!($T => {
      ($SELF) -> $COLLECTION<&$ITEM> {
        { $($PRE)+ $($ASREF)+ $($POST)+ }
      }
    });
    crate::macros::IntoIterator!($T => {
      ($SELF) -> $COLLECTION<$ITEM> {
        { $($PRE)+ $($OWNED)+ $($POST)+ }
      }
    });
  };

  ($T:ty => {
    ($SELF:ident) -> $COLLECTION:ident<$ITEM:ty> {
      { $($PRE:tt)+ } => FilterMap [$($($INNER:ident).*),*] $($POST:tt)+
    }
  }) => {
    crate::macros::IntoIterator!($T => {
      ($SELF) -> $COLLECTION<$ITEM> {
        { $($PRE)+ } => where {
          AsRef => { .iter() $(.filter_map(|this| this.$($INNER).*.as_ref()))+ },
          Owned => { .into_iter() $(.filter_map(|this| this.$($INNER).*))+ },
        } $($POST)+
      }
    });
  };
}

pub(crate) use IntoIterator;
