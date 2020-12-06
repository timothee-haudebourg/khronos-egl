# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Changed
- Removed the `khronos` dependency.

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