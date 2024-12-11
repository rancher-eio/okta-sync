// use divan::Bencher;
// use eio_enterprise_migrator::okta::api::v1::users::ListUsers;
// use eio_enterprise_migrator::okta::{filter, pagination};
// use eio_enterprise_migrator::traits::query_string::{NestedQueryString, QueryString};

// const BADNESS: &str = "bad data in benchmark suite";

fn main() {
  divan::main();
}

// fn url() -> Url {
//   Url::parse("https://suse.okta.com/api/v1/users/").expect(BADNESS)
// }

// fn api() -> ListUsers {
//   ListUsers {
//     filter: filter::Options { filter: None },
//     pagination: pagination::Options {
//       after: None,
//       before: None,
//       limit: Some(200),
//     },
//     q: Some(String::from("query")),
//     search: Some(String::from("search")),
//     sort_by: Some(String::from("ascending")),
//     sort_order: Some(String::from("email")),
//   }
// }

// #[divan::bench]
// fn nested(bencher: Bencher) {
//   bencher
//     .with_inputs(|| api())
//     .bench_refs(|api| NestedQueryString::nested_query_string(api).expect(BADNESS));
// }

// #[divan::bench]
// fn flat(bencher: Bencher) {
//   bencher
//     .with_inputs(|| api())
//     .bench_refs(|api| QueryString::query_string(api).expect(BADNESS));
// }
