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

- A `Config` trait with default implementations for parsing configuration from strings in multiple formats (e.g., TOML, JSON).  
- A `Validate` trait for implementing custom validation logic.  
- Clear and descriptive error handling via `ConfigError`.  
- Basic schema generation by printing default values as examples.

---

## Getting Started

To use `invariance` in your Rust project, add it to your `Cargo.toml` using:

```bash
cargo add invariance
```

---

## Features & Roadmap

- [x] **Strongly typed config parsing** from TOML and JSON strings  
- [x] **Custom validation** with clear error reporting  
- [x] **Trait-based design** with minimal boilerplate  

---

- [ ] Support for additional formats: **YAML**  
- [ ] CLI tools for **config validation**  
- [ ] Macro-driven **validation DSL**  
- [ ] Schema **generation and introspection**