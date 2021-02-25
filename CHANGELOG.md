# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unpublished]
### Changed
- Upgrade dependency `libloading`: ^0.6 -> ^0.7
- Removed the `nightly` feature

## [3.0.1]
### Changed
- Load `libEGL.so.1` by default instead of `libEGL.so`.

## [3.0.0]
### Changed
- Impl `Debug` for `Static`, `Dynamic` and `Instance`.
- Add a `DynamicInstance` type alias for `Instance<Dynamic<libloading::Library>>` with helper functions.
- Precise version selection.
- Dynamic cast between versions with `Dynamic::load`, `Dynamic::load_required` and the `Upcast`/`Downcast` traits.
- `DynamicInstance::downcast` and `IDynamicInstance::upcast`.

## [3.0.0-beta]
### Changed
- Removed the `khronos` dependency.
- Dynamic linking: Add the `Api` trait and the `Instance` struct along with the `static` and `dynamic` features.
- The dependency to `pkg-config` is now optional, only required by the `static` feature.
- Add an optional dependency to `libloading`, only required by the `dynamic` feature.

## [2.2.0]
### Added
- Fix #9: new function `get_config_count` to get the number of available frame buffer configurations.

## [2.1.1]
### Changed
- Upgrade dependency `gl`: ^0.11 -> ^0.14
- Upgrade dependency `wayland-client`: ^0.23 -> ^0.25
- Upgrade dependency `wayland-protocols`: ^0.23 -> ^0.25
- Upgrade dependency `wayland-egl`: ^0.23 -> ^0.25

## [2.1.0]
### Changed
- Fix #3: accept `Option<Display>` instead of `Display` in `query_string`.
- More flexible dependencies versions.
