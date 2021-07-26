# Changelog

All notable changes to osoy will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0),
and this project adheres to
[Semantic Versioning](https://doc.rust-lang.org/cargo/reference/semver.html).

## [Unreleased]

### Added

- `--parallel` flag to pull & clone operations which denotes the number of
  parallel jobs
- `--verbose` flag to pull & clone operations
- `--force` flag to pull operation

### Fixed

- Ensure that regular expressions are compiled only once

## [0.4.0] - 2021-02-18

### Added

- Support for importing as a library
- Support for extensions

## [0.3.1] - 2021-02-11

### Fixed

- Renaming only remote origin protocol

## [0.3.0] - 2021-01-09

- Full rewrite

[unreleased]: https://gitlab.com/osoy/osoy/compare/v0.4.0...master
[0.4.0]: https://gitlab.com/osoy/osoy/compare/v0.3.1...v0.4.0
[0.3.1]: https://gitlab.com/osoy/osoy/compare/v0.3.0...v0.3.1
[0.3.0]: https://gitlab.com/osoy/osoy/tree/v0.3.0
