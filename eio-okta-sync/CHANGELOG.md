# Changelog

## 0.7.2

- (CHANGE) renamed field in mappings: ignoredUsers -> excludeUsers (old name still supported for compatibility)
- (FEATURE) added support for including users in mappings regardless of group membership (use `includeUsers: {}`)
- (FEATURE) added support for matching Okta IDs in mappings excludeUsers/includeUsers (use `oktaIds: []`)
- (DEPENDENCY) eio-okta-client 0.4.0 -> 0.4.1
- (DEPENDENCY/ADD) using bon in eio-okta-sync
- (DEPENDENCY/ADD) using chrono in eio-okta-sync

## 0.7.1

- (DEPENDENCY) eio-okta-client 0.3.0 -> 0.4.0
- (DEPENDENCY) inquire 0.7.5 -> 0.9.1
- (DEPENDENCY) octocrab 0.44.1 -> 0.48.1

## 0.7.0

- (FEATURE) added org-graph commands for working with snapshots as graph data.
  - (FEATURE) "org-graph dot" can produce DOT output for rendering with GraphViz.
  - (FRATURE) "org-graph path" can map paths to users within the org hierarchy.
- (INTERNAL) updated UserProfileExtensions to include additional fields.
- (INTERNAL) unified snapshot loading code.

## 0.6.0

- (FEATURE) Okta snapshots now support custom properties in user profiles.
- (INTERNAL) fixed several clippy lints

## 0.5.2

- (FEATURE) generated Team and TeamMembership resources now support --embed-github-org-name.
- (FEATURE) added option to override the crossplane provider config name.

## 0.5.1

- (FEATURE) added --valid-github-username option to "generate" command.
- (FEATURE) added ignoredUsers field to mappings config.
- (FEATURE) added support for generating ignoredUsers to "mappings" command.
- (LIB) new `Error` variant: `Error::FancyRegex`.
- (INTERNAL) Mappings and Expectations now implement Default.
- (DEPENDENCY/ADD) using educe in eio-okta-sync
- (DEPENDENCY/NEW) added fancy-regex

## 0.5.0

- (FEATURE) added --all-users option to "generate" command.
- (BREAKING/COMMAND) - "generate" now sources users from `$.group_users[*]` in snapshot, where `*` is `$.expectations.oktaGroup[].id` in the mapping config.
  - (BREAKING/MITIGATION) - set --all-users=true to read from `$.users` in snapshot, as was the default behaviour in prior versions.

## 0.4.1

- (BUGFIX) fix --kind option for "make-archive" command

## 0.4.0 (yanked)

- (FEATURE) added --force-yaml-start-of-document and --force-yaml-end-of-document options to "make-archive" command.
  - these options can be used to force the YAML output to include explicit start/end markers.

## 0.3.2

- (BREAKING/BUILD) Minimum Supported Rust Version increased from 1.81 to 1.83
  - (BREAKING/MIGRATION) upgrade your rust toolchain

## 0.3.1 (yanked)

- (YANKED) MSRV is incorrect.
- (BUGFIX) "make-archive" now produces tarballs with modes that allow reading!
- (FEATURE) "make-archive", "generate", and "current" all support multiple ways to embed and consume GitHub Org names in resources, rather than forcing it as a name prefix.

## 0.3.0

- (FEATURE) new command "make-archive" produces a filesystem tarball from a generated resources input.
- (FEATURE) mappings subcommand has a description now
- (DEPENDENCY/NEW) added tar

## 0.2.1

- (FIX) fixed interactive prompts in mapping subcommand.

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
| BUGFIX             | something was wrong before, and now it isn't.
| DEPENDENCY/ADD     | added a dependency to this crate that is already used in the project.
| DEPENDENCY/NEW     | added a dependency to the project.
| INTERNAL           | noted for reference, but does not affect public interfaces.
| LIB                | changes something about the library in a backwards-compatible way.
| BREAKING/BUILD     | changes something about the build and requires changes.
| BREAKING/COMMAND   | changes something about the a command's default behaviour.
| BREAKING/LIB       | changes something about the library and requires changes.
| BREAKING/MIGRATION | generally paired with a BREAKING change, this is what to do about it.
| FEATURE            | adds a feature.
| LICENSE            | change relating to licensing.
| YANKED             | version was published, then un-published.
