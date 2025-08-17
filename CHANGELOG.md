# Changelog

All notable changes to this project will be documented in this file.

## [0.1.0] - 17-08-2025

### Added

- Initial release of `invariance` crate.
- `Config` trait with default `from_str` method for parsing from TOML.
- `Validate` trait for implementing config validation logic.
- Custom error type `ConfigError` for descriptive validation and parsing errors.
- Example demonstrating safe, validated config parsing.
- Basic schema printing functionality based on `Default` trait.