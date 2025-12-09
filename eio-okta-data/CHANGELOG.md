# Changelog

## 0.5.0

- (BREAKING) replaced `derive_builder` with `bon`
  - this replaces an entire class of runtime errors with compile-time checks.
  - setters no longer accept `impl Into<T>`, except for `String` fields.
    - the old behaviour can be reintroduced selectively as needed.
- (BREAKING/MITIGATION) use `T::builder().set(value).build()` instead of `T::default().set(value).build().unwrap()`.

## 0.4.1

- (DEPENDENCY) proptest_derive 0.6.0 -> 0.7.0
- (DEPENDENCY) mediatype 0.20.0 -> 0.21.0

## 0.4.0

- (FEATURE) UserProfile now supports custom properties.
- (BREAKING) changed fields on UserProfile
  - removed non-standard github_orgs / github_username fields.
  - added `_extensions: BTreeMap<String, Value>` field to capture all custom properties
  - added `extensions_into<T: DeserializeOwned>()` method for deserializing into custom types.
- (BREAKING/MITIGATION) use `_extensions` or `extensions_into()` to read custom fields.

## 0.3.0

- (BREAKING BUGFIX) add missing `UserStatus` variant: `UserStatus::Recovery`.
  - (BREAKING/MIGRATION) add another arm to any match statements that are matching on `UserStatus` variants.

## 0.2.1

- (COMPATIBILITY) impl CityNameGenFn for fake::locales::Custom
  - required for compatibility with fake v3.1.0+

## 0.2.0

- (LICENSE) more flexible licensing, changed from `MIT` to `MIT OR Apache-2.0`.

## v0.1.0 - Initial Release
