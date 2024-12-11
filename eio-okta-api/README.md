# okta-api - API Types for Okta

This crate contains API definitions for working with the Okta API.

Notably, it does not provide an implementation of anything that *uses* these types.

This crate is intended to be used to to produce and validate tools that are compatible with the the Okta API, either server or client implementations.

At this point, the types are hand-written, and even those on an as-needed basis (Pull Requests welcome!). A longer-term goal of this crate is to be able to be generated from the [Okta Management API OpenAPI Specification](https://github.com/okta/okta-management-openapi-spec).
