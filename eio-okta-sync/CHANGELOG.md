# Changelog

## 0.2.0

- (BREAKING/LIB) new `Error` variant: `Error::Inquire`.
  - (BREAKING/MIGRATION) - add another arm to any match statements that are matching on error variants.
- (BREAKING/LIB) `Error` is now non-exhaustive.
  - (BREAKING/MIGRATION) - add an else (`_`) arm if you're matching on error variants.
- (FEATURE) new command "mappings" to interactively generate mappings config from snapshot data.
- (FEATURE) new command "check-version" can be used to check for available updates.
- (LICENSE) more flexible licensing, changed from `MIT` to `MIT OR Apache-2.0`.
- (LIB) `github::Membership` now implements `Hash`
- (INTERNAL) `Snapshot` and related data structures now implement `Hash`, some also implement `Display`.
- (INTERNAL) replaced `std::fs::File` with `fs_err::File` for better error reporting.
- (DEPENDENCY/ADD) using constcat in eio-okta-sync
- (DEPENDENCY/ADD) using humantime in eio-okta-sync
- (DEPENDENCY/NEW) added console
- (DEPENDENCY/NEW) added heck
- (DEPENDENCY/NEW) added inquire
- (DEPENDENCY/NEW) added itertools
- (DEPENDENCY/NEW) added semver
- (DEPENDENCY/NEW) added update-informer

## v0.1.0 - Initial Release

## Tag Glossary

| Tag                | Description
|--------------------|------------
| DEPENDENCY/ADD     | added a dependency to this crate that is already used in the project.
| DEPENDENCY/NEW     | added a dependency to the project.
| INTERNAL           | noted for reference, but does not affect public interfaces.
| LIB                | changes something about the library in a backwards-compatible way.
| BREAKING/LIB       | changes something about the library and requires changes.
| BREAKING/MIGRATION | generally paired with a BREAKING change, this is what to do about it.
| FEATURE            | adds a feature.
| LICENSE            | change relating to licensing.
