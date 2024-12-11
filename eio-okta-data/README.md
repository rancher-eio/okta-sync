# okta-data - Data Types for Okta

This crate contains data types only.

## Conventions

In the details below, the following conventions are assumed:

- `{Type}` is intended as the type identifier of some type.
  - for example, `{Type}Status` should be understood as `FooStatus` in relation to type `Foo`, and `BarStatus` in relation to type `Bar`.
- `{field}` is intended as the named identifier of a struct field.
  - example: `fn set_{field}(&mut self, value: T)` should be understood as `fn set_foo(&mut self)` in relation to a field named `foo` with type `T` on some struct. It's probably fair to assume that such a function sets the `foo` field to whatever `value` was given.
- `(...)` is intended to be understood as "a tuple with some number of members", the number likely being fixed, but unspecified in the example.
- "the prelude" refers to the `prelude` module at the root of the crate.

## Features

As a loose guideline, most features are named for the crate it adds integrations for, or the attribute it adds to a type.

- `arbitrary`
  - implements `arbitrary::Arbitrary` for all types, to support fuzzing tools like `cargo-fuzz` and `AFL`.
- `builder`
  - adds a `{Type}Builder` struct alongside all struct types.
    - this provides a builder-style API for constructing data.
    - this includes type conversion from anything that implements `Into` for the corresponding field type.
  - adds a `{Type}BuilderError` struct.
- `clap`
  - implements `clap::Args` for all struct types, allowing fields to be read from CLI options/flags.
  - implements `clap::ValueEnum` for all enum types, allowing use and validation as CLI args.
- `comparable`
  - adds a `{Type}Change` enum alongside all struct types.
  - adds a `{Type}Desc` struct alongside all struct types.
  - implements `comparable::Comparable` for all types.
    - this adds a `fn comparison(&self, other: &Self) -> Vec<{Type}Change>` function to all types.
    - this adds support for analyzing the changes between two instances of any data type.
- `dissolve`
  - adds a `fn dissolve(self) -> (...)` method to all struct types.
    - this consumes the struct and returns a tuple of all fields (in declaration order).
      - to mitigate breaking changes here, all structs fields are strictly sorted.
        - this is verified and enforced at compile time thanks to [`remain`](https://crates.io/crates/remain).
- `dummy`
  - implements `fake::Dummy` for all types, allowing generation of plausible examples.
- `getter`
  - adds a `fn {{field}}(&self) -> &T` method for each field of all struct types.
    - this provides immutable read-only references to the data.
    - for `Copy` types, this returns a copy of the data instead.
- `proptest`
  - implements `proptest::Arbitrary` for all types, which in turn enables property-based testing with automatic shrinking.
- `schemars`
  - implements `schamars::JsonSchema` for all types
    - this allows any type to produce schemas for validation with other (potentially non-Rust) programs.
    - this also allows crates like `kube` to generate Kubernetes Controllers and such from these types.
- `serde`
  - implements `serde::Serialize` and `serde::Deserialize` for all types.
    - this allows conversion to/from any serde-compatible serializer (which is most of them).
- `patch`
  - adds a `{Type}Patch` struct alongside all struct types.
    - this is a mirror of `{Type}`, with the notable difference that all fields are optional.
    - this also implements `Default`, even if `{Type}` doesn't (default being an empty patch).
  - implements `struct_patch::Patch` for all struct types.
    - this adds support for applying partial updates to the data.
  - if the `serde` feature is enabled, `{Type}Patch` is (de)serializable.
  - if the `comparable` feature is enabled, `{Type}Patch` implements `comparable::Comparable`.
  - if the `builder` feature is enabled, adds a `{Type}PatchBuilder` struct.
- `strum`
  - implements all `derive`-able traits from the `strum` crate for all enum types.
- `validate`
  - implements `validator::Validate` for all struct types.
  - implements `validator::Validate` for all enum types.
    - this will always return `Ok(())` for enum variants that do not contain data.
      - For those that *do* contain data, it simply calls `.validate()` on the data and returns the result.

### (Partially) Overlapping Features

`arbitrary`, `fake`, and `proptest` have some overlap, in that they all allow generation of arbitrary samples for supported types. Where they differ is the primary use case.

`fake::Dummy` is intended to generate human-friendly samples for the purpose of testing, documentation, mocking, and prototyping applications. Additional effort has been made to have these generators produce *plausible* examples. For example, a postal address generated here is likely to resemble a realistic address, even if the address doesn't exist, it should be something that looks like it could. They allow you to build using realistic data without requiring access to any service. As such, they are often useful for unit testing, but less so for integration testing.

`proptest::Arbitrary` is intended to be driven via `cargo test`, generating hundreds (or thousands) of randomized inputs for each test, and when things fail, retrying with progressively simpler variants of the failing input until it isolates a minimal reproducible failure, and then producing a comittable regression file to ensure that failure is always tested going forward.

`arbitrary::Arbitrary` however, is intended to be driven with `cargo fuzz`, potentially generating an exhaustive set of all possible inputs (given infinite time). The purpose here is using input fuzzing to guide automated exploration and analysis of potential execution paths in the program.

`comparable` and `struct_patch` also have some overlapping functionality, but the intended use is considerably different.

`comparable::Comparable` is intended for diagnostics, testing, and verification. It produces granular, iterable changesets with human-friendly descriptions of those changes, making it straightforward to identify small changes in deeply nested structures. `{Type}Change` and `{Type}Desc` ensure that there is a representation for all possible changes that could occur to a given data type. The `comparable` crate also provides a powerful `assert_changes!` macro that uses these to define test expectations, and is very useful when testing mutable operations.

`struct_patch::Patch` on the other hand, is intended to be used for **transforming** data. `{Type}Patch` can be used to apply partial changes to the original type (for example, redacting sensitive information). These patches can be added together and/or merged, allowing complex patching processes to be composed from simple, testable fragments. It is also possible to take any two instances of a patchable data type and produce a strongly-typed patch that transforms one into the other. This in turn provides the core functionality for **staging** and **undoing** changes.

### Notes on Patching Optional Fields

If the `patch` feature is enabled, the `{Type}Patch` struct may feel slightly counterintuitive, when `{Type}` has fields that are `Option<T>`.

```rust
use okta_data::v2024_07_0::management::components::schemas::{
  user_profile::{UserProfilePatch, UserProfilePatchBuilder},
};

let patch = UserProfilePatchBuilder::default()
  .employee_number(Some(Some(String::from("31337"))))
  .build().unwrap();
```

The `Option<Option<T>>` there is the part that feels a bit weird, right? The reason it works this way is because we need to distinguish between "this patch does not apply to this field" (`None`), and "this patch applies to this field, and is explicitly setting it to `None` (which we represent as `Some(None)`). As a result, setting it to a specific value `Some(Some(value))`, which is visually unappealing, but logically coherent.

### Limitations

Some fields like `User::embedded` do not have a fixed definition in the [Okta Managment API OpenAPI Specification](https://github.com/okta/okta-management-openapi-spec), and so are represented as `serde_json::Value`, so you can define your own conversions for whatever sort of data you happen to put in there.

However, given that these fields are unspecified, they're also untestable within the scope of this crate. As such, several features (notably testing-oriented ones such as `comparable`, `proptest`, `dummy`, and `arbitrary`) simply ignore these fields.
