# invariance

[![Crates.io](https://img.shields.io/crates/v/invariance.svg)](https://crates.io/crates/invariance)
[![Docs.rs](https://docs.rs/invariance/badge.svg)](https://docs.rs/invariance)
[![Build Status](https://github.com/ParkBlake/invariance/actions/workflows/ci.yml/badge.svg)](https://github.com/ParkBlake/invariance/actions)
[![License](https://img.shields.io/crates/l/invariance.svg)](https://github.com/ParkBlake/invariance/blob/master/LICENSE)

A Rust crate providing **type-safe, validated configuration** for preventing runtime crashes caused by invalid config data.

---

## Overview

`invariance` helps you define configuration structs with strong typing and validation logic. It ensures that configurations loaded from files (currently TOML and JSON) are **always validated before use**, preventing null pointer errors, invalid states, and unexpected runtime failures.

The crate provides:

- A `Config` trait with default implementations for parsing configuration from strings in multiple formats (currently TOML and JSON), enabling flexible config loading without boilerplate.
- A `Validate` trait for implementing custom, domain-specific validation logic to enforce invariants and prevent invalid config states at runtime.  
- Clear and descriptive error handling with the `ConfigError` type, supporting error chaining and categorisation for easier debugging and reporting.  
- Basic schema generation by printing default configuration instances as example outputs in both JSON and TOML formats.


## Getting Started

To use `invariance` in your Rust project, add it to your `Cargo.toml` using:

```bash
cargo add invariance
```

---

## Features & Roadmap

- [x] **Strongly typed config parsing** from TOML and JSON strings
- [x] **Custom validation** via a trait-based system with clear reporting
- [x] **Minimal boilerplate** thanks to trait defaults and composable design

---

- [ ] Support for additional serialization formats, starting with **YAML**  
- [ ] CLI tools for **config validation** and schema inspection  
- [ ] Macro-driven **validation DSL** to simplify and standardise validation logic  
- [ ] Advanced schema **generation and introspection** features for improved developer experience