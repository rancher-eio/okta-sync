# eio-okta-api - API Types for Okta

This crate contains API definitions for working with the Okta API.

Notably, it does not provide an implementation of anything that *uses* these types.

This crate is intended to be used to to produce and validate tools that are compatible with the the Okta API, either server or client implementations.

At this point, the types are hand-written, and even those on an as-needed basis (Pull Requests welcome!). A longer-term goal of this crate is to be able to be generated from the [Okta Management API OpenAPI Specification](https://github.com/okta/okta-management-openapi-spec).

## License

```text
SPDX-License-Identifier: MIT OR Apache-2.0
```

`eio-okta-api` is available under [your choice](https://fossa.com/blog/dual-licensing-models-explained/) of *either* the [MIT License](https://colstrom.mit-license.org) *or* the [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0).

See [`LICENSE-MIT`](../LICENSE-MIT) and [`LICENSE-APACHE`](../LICENSE-APACHE) at the root of the repository for the full text of each.

Both are written in fancy lawyer-speak. If you prefer more down-to-earth language, consider the following:

- tl;drLegal has simple visual summaries available: [`MIT`](https://www.tldrlegal.com/license/mit-license) or [`Apache-2.0`](https://www.tldrlegal.com/license/apache-license-2-0-apache-2-0)
- FOSSA has more in-depth overviews available: [`MIT`](https://fossa.com/blog/open-source-licenses-101-mit-license/) or [`Apache-2.0`](https://fossa.com/blog/open-source-licenses-101-apache-license-2-0/)
