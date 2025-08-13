# Changelog

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
