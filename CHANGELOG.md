# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

## [v0.9.2] - 2021-07-17

### Changed

- Fixed `e310x-hal::delay::Delay` timing typo with extra 0

## [v0.9.1] - 2021-07-15

### Added

- Added implementation of `embedded_hal::blocking::delay::DelayUs` for `e310x-hal::delay::Delay` using `MTIME`
