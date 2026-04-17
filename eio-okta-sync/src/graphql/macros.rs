macro_rules! PageSize {
  ($T:ty as GraphQLQuery) => {
    const _: () = {
      use crate::graphql::PageSize;
      use ::graphql_client::GraphQLQuery;

      impl PageSize for $T where Self: GraphQLQuery {}
    };
  };
}

pub(crate) use PageSize;

macro_rules! EndCursor {
  ($T:ty as GraphQLQuery => ResponseData => PageInfo { $ident:ident }) => {
    const _: () = {
      use ::graphql_client::GraphQLQuery;
      use crate::graphql::{EndCursor, PageInfo};

      impl EndCursor for <<$T as GraphQLQuery>::ResponseData as PageInfo>::PageInfo {
        fn end_cursor(&self) -> Option<&str> {
          Some(self.$ident.as_ref()?.as_str())
        }
      }
    };
  };

  ($T:ty as GraphQLQuery => ResponseData => PageInfo) => {
    crate::graphql::macros::EndCursor!(
      $T as GraphQLQuery => ResponseData => PageInfo { end_cursor }
    );
  };
}

pub(crate) use EndCursor;

macro_rules! HasNextPage {
  ($T:ty as GraphQLQuery => ResponseData => PageInfo { $ident:ident }) => {
    const _: () = {
      use ::graphql_client::GraphQLQuery;
      use crate::graphql::{HasNextPage, PageInfo};

      impl HasNextPage for <<$T as GraphQLQuery>::ResponseData as PageInfo>::PageInfo {
        fn has_next_page(&self) -> bool {
          self.$ident
        }
      }
    };
  };

  ($T:ty as GraphQLQuery => ResponseData => PageInfo) => {
    crate::graphql::macros::HasNextPage!(
      $T as GraphQLQuery => ResponseData => PageInfo { has_next_page }
    );
  };
}

pub(crate) use HasNextPage;

macro_rules! After {
  ($T:ty as GraphQLQuery => Variables { $ident:ident }) => {
    const _: () = {
      use crate::graphql::After;
      use ::graphql_client::GraphQLQuery;

      impl After for <$T as GraphQLQuery>::Variables {
        fn after(&mut self) -> &mut Option<String> {
          &mut self.$ident
        }
      }
    };
  };

  ($T:ty as GraphQLQuery => Variables) => {
    crate::graphql::macros::After!(
      $T as GraphQLQuery => Variables { after }
    );
  };
}

pub(crate) use After;

macro_rules! First {
  ($T:ty as GraphQLQuery => Variables { $ident:ident }) => {
    const _: () = {
      use crate::graphql::First;
      use ::graphql_client::GraphQLQuery;

      impl First for <$T as GraphQLQuery>::Variables {
        fn first(&mut self) -> &mut Option<i64> {
          &mut self.$ident
        }
      }
    };
  };

  ($T:ty as GraphQLQuery => Variables) => {
    crate::graphql::macros::First!(
      $T as GraphQLQuery => Variables { first }
    );
  };
}

pub(crate) use First;

macro_rules! ForwardPageVariables {
  ($T:ty as GraphQLQuery => Variables) => {
    crate::graphql::macros::After!($T as GraphQLQuery => Variables);
    crate::graphql::macros::First!($T as GraphQLQuery => Variables);
  };
}

pub(crate) use ForwardPageVariables;

macro_rules! ForwardPageInfo {
  ($T:ty as GraphQLQuery => ResponseData => PageInfo) => {
    crate::graphql::macros::EndCursor!($T as GraphQLQuery => ResponseData => PageInfo);
    crate::graphql::macros::HasNextPage!($T as GraphQLQuery => ResponseData => PageInfo);
  };
}

pub(crate) use ForwardPageInfo;

macro_rules! ForwardPagination {
  ($T:ty as GraphQLQuery) => {
    crate::graphql::macros::PageSize!(
      $T as GraphQLQuery
    );
    crate::graphql::macros::ForwardPageInfo!(
      $T as GraphQLQuery => ResponseData => PageInfo
    );
    crate::graphql::macros::ForwardPageVariables!(
      $T as GraphQLQuery => Variables
    );
  };
}

pub(crate) use ForwardPagination;
