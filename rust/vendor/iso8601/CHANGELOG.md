# Changelog

<!-- next-header -->

## [Unreleased](https://github.com/badboy/iso8601/compare/v0.6.3...main) - ReleaseDate

## [0.6.3](https://github.com/badboy/iso8601/compare/v0.6.2...v0.6.3) - 2025-05-15

## [0.6.2](https://github.com/badboy/iso8601/compare/v0.6.1...v0.6.2) - 2025-02-02

## [0.6.1](https://github.com/badboy/iso8601/compare/v0.6.0...v0.6.1) - 2023-02-12

## [0.6.0](https://github.com/badboy/iso8601/compare/v0.5.1...v0.6.0) - 2023-02-12

* add serde (de)serializer implementations
* add conversion to chrono as a feature

## [0.5.1](https://github.com/badboy/iso8601/compare/v0.5.0...v0.5.1) - 2022-11-09

* Fix accepted duration representations

## [0.5.0](https://github.com/badboy/iso8601/compare/v0.4.2...v0.5.0) - 2022-07-29

* Replace rounding-error prone floating point code with robust integer code ([#36](https://github.com/badboy/iso8601/pull/36) by @plugwash)
* Make low-level parsers public ([c80b169](https://github.com/badboy/iso8601/commit/c80b169c53716d63e4d56a9c10775a931d6ce0be))

## [0.4.2](https://github.com/badboy/iso8601/compare/v0.4.1...v0.4.2) - 2022-06-01
* Fix TZ offset minutes being replaced by hours

## [0.4.1](https://github.com/badboy/iso8601/compare/v0.4.0...v0.4.1) - 2021-11-21
* Add `Display` implementations for more exported structures (Duration)

## [0.4.0](https://github.com/badboy/iso8601/compare/v0.3.0...v0.4.0) - 2020-02-27

* Upgrade to [nom 5](http://unhandledexpression.com/general/2019/06/17/nom-5-is-here.html), getting rid of all parser macros ([#22](https://github.com/badboy/iso8601/pull/22)).
* Added support for ISO 8601 Durations ([#24](https://github.com/badboy/iso8601/pull/24), thanks to @zoewithabang).

## [0.3.0](https://github.com/badboy/iso8601/compare/v0.2.0...v0.3.0) - 2019-01-31

* Add `Display` implementations for exported structures
* Implemented `FromStr` for `Date`, `Time` and `DateTime`
* Upgraded to nom 4
* Formatted everything with `rustfmt`

## [0.2.0](https://github.com/badboy/iso8601/compare/v0.1.0...v0.2.0) - 2017-11-06

* Upgraded nom
* Added fuzzing targets
* Correctly overwrite hours and minutes in `Time#set_tz`
* Correct small error in README example

## 0.1.0 - 2017-11-06

Initial release
