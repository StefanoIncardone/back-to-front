# Change Log

> [!WARNING]
> Changes can happen at any moment for now, use at your own risk

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html),
but may switch to [CalVer Versioning](https://calver.org/) in the future.

## Unrealeased

- More ergonomic digits global constants and bases modules
- Index iterators
- More type safe `Base` enum with all possible bases, which would allow for the safe removal of
    base min and max errors, constants and enum variants

## Known issues

- Digit ranges macros are also importable from the root module

## 0.1.1 -

### Added

- Modularized digits ranges and added more usefull functions, macros and constants
- Added `check_*_offset` and `parse_*_offset`:
    - introduced `INVALID`, `OUT_OF_RANGE`, `BASE_MIN` and `BASE_MAX` constants
    - introduced `Offset`, `OffsetCustomBase`, `DigitOffset` and `DigitOffsetCustomBase` aliases

### Changed

- Updated Rust version to 1.89.0
- Marked `check_*` and `parse_*` and related enums `AsciiDigit`, `Digit`, `AsciiDigitCustomBase` and
    `DigitCustomBase` as deprecated, to avoid inconsistencies
- Marked checking and parsing of tally counters as deprecated

## 0.1.0 - 2025/08/07

### Added

- Ascii digit checking and parsing of tally counters, binary, octal, decimal, hexadecimal and
    custom bases
- `Span` struct
- x86_64 registers enums
