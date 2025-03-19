# Changelog

## 0.3.0

- (BREAKING BUGFIX) add missing `UserStatus` variant: `UserStatus::Recovery`.
  - (BREAKING/MIGRATION) add another arm to any match statements that are matching on `UserStatus` variants.

## 0.2.1

- (COMPATIBILITY) impl CityNameGenFn for fake::locales::Custom
  - required for compatibility with fake v3.1.0+

## 0.2.0

- (LICENSE) more flexible licensing, changed from `MIT` to `MIT OR Apache-2.0`.

## v0.1.0 - Initial Release
