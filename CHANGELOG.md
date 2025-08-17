# Changelog

All notable changes to this project will be documented in this file.

## [0.1.1] - 17-08-2025

### Added

- Support for parsing configuration from JSON strings.
- Improved `Config` trait to handle multiple formats.

### What has changed

- Added support for parsing configuration from both TOML and JSON strings (`from_toml_str` and `from_json_str`).
- Validation remains explicitly separate from parsing, making error handling and propagation clearer.
- Improved documentation and examples to reflect the new multi-format parsing capabilities and enhanced error clarity.

## [0.1.0] - 17-08-2025

### Added

- Initial release of `invariance` crate.
- `Config` trait with default `from_str` method for parsing from TOML.
- `Validate` trait for implementing config validation logic.
- Custom error type `ConfigError` for descriptive validation and parsing errors.
- Example demonstrating safe, validated config parsing.
- Basic schema printing functionality based on `Default` trait.
