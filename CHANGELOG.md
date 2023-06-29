# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

- Use `atomic-polyfill` to allow builds on riscv32imc-unknown-none-elf targets when needed.

## [v0.10.0] - 2023-03-28

### Added
- Added Pulse Width Modulation interface implementing `embedded_hal::Pwm`
- Added `interrupt` module for vectored interrupt handlers. This module is only active if feature `virq` is selected.

### Changed
- Refactored `e310x-hal::spi` module, splitting the abstraction into `SpiBus` and `SpiExclusiveDevice/SpiSharedDevice` to allow multiple devices on a single SPI bus to co-exist
- Update `e310x` dependency to version 0.11
- Update `riscv` dependency to version 0.10

### Removed
- removed interrupt linking definitions, they are now provided by `e310x` via `svd2rust`

## [v0.9.4] - 2022-07-10

### Changed

- Fixed code still using old `riscv::interrupt::Nr`

## [v0.9.3] - 2021-08-15

### Changed

- Fixed `e310x-hal::delay::Delay` call typo to `delay_ms`

## [v0.9.2] - 2021-07-17

### Changed

- Fixed `e310x-hal::delay::Delay` timing typo with extra 0

## [v0.9.1] - 2021-07-15

### Added

- Added implementation of `embedded_hal::blocking::delay::DelayUs` for `e310x-hal::delay::Delay` using `MTIME`
