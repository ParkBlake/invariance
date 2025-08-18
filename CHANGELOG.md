# Changelog

All notable changes to this project will be documented in this file.

## [0.1.2] - 17-08-2025

### Added

- Added `parse_config_file` utility to read and parse config files based on file extension or explicit format hint, supporting TOML and JSON.
- Extended `Config` trait with distinct `from_toml_str` and `from_json_str` methods for clearer multi-format support.
- Enhanced `ConfigError` with `source` and `kind` fields to provide better error context and categorisation.
- Added `print_schema` function to output example configurations in both JSON and TOML formats, aiding users in understanding config shape.
- Improved error messages to include error kind tags for easier debugging.

### What has changed

- Refined separation between parsing and validation, making error handling and propagation clearer.
- Modularised parsing logic via a dedicated parser utility.
- Strengthened documentation and inline comments.
- Added support for serialising default configs to both JSON and TOML as usage references (not formal schemas yet).
- Cleaned up trait bounds and implementations.

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
