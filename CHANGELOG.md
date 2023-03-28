# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

## [v0.11.0]

### Changed
- Update `riscv` dependency to version 0.10
- Regenerate code with `svd2rust` v0.26.0

## [v0.10.0] - 2022-09-04

### Added

### Changed

- Update `riscv` dependency to version 0.8
- Regenerate code with `svd2rust` v0.19.0

### Fixed

- Fix QSPI `delay0` and `delay1` reset values


## [v0.9.0] - 2020-11-01

### Changed

- Update `riscv` dependency to version 0.6
- Rename QSPI registers and fields to match the datasheet

### Fixed

- Fix QSPI `ffmt.pad_cnt` field definition


[Unreleased]: https://github.com/riscv-rust/e310x/compare/v0.10.0..HEAD
[v0.10.0]: https://github.com/rust-embedded/riscv-rt/compare/v0.9.0...v0.10.0
[v0.9.0]: https://github.com/riscv-rust/e310x/compare/v0.8.1...v0.9.0
