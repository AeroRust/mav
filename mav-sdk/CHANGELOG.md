# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Feature `with_serde` - if enabled it will add `#[derive(Serialize, Deserialize)]` to all `...Request` & `...Response` types as well as all inside types.

### Changed

- Updated `mavsdk-proto` git submodule
- Updated `tonic` (0.8), `tonic-build` (0.8) and `prost` (0.11)

## [0.1.0] - 2021-09-04

Initial release

[Unreleased]: https://github.com/olivierlacan/keep-a-changelog/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/olivierlacan/keep-a-changelog/releases/tag/v0.1.0