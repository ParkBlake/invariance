# invariance

[![Crates.io](https://img.shields.io/crates/v/invariance.svg)](https://crates.io/crates/invariance)
[![Docs.rs](https://docs.rs/invariance/badge.svg)](https://docs.rs/invariance)
[![Build Status](https://github.com/ParkBlake/invariance/actions/workflows/ci.yml/badge.svg)](https://github.com/ParkBlake/invariance/actions)
[![License](https://img.shields.io/crates/l/invariance.svg)](https://github.com/ParkBlake/invariance/blob/master/LICENSE)

A Rust crate providing **type-safe, validated configuration** for preventing runtime crashes caused by invalid config data.

---

## Overview

`invariance` helps you define configuration structs with strong typing and validation logic. It ensures that configuration loaded from files (currently TOML) is **always validated before use**, preventing null pointer errors, invalid states, and unexpected runtime failures.

The crate provides:

- A `Config` trait with a default implementation for parsing configuration from strings.  
- A `Validate` trait for implementing custom validation logic.  
- Clear and descriptive error handling via `ConfigError`.  
- Basic schema generation by printing default values as examples.

---

## Getting Started

Add `invariance` to your `Cargo.toml` in your project.

```bash
cargo add invariance
```

---

## Features & Roadmap

- [x] **Strongly typed config parsing** from TOML strings  
- [x] **Custom validation** with clear error reporting  
- [x] **Trait-based design** with minimal boilerplate  

---

- [ ] Support for additional formats: **JSON, YAML**  
- [ ] CLI tools for **config validation**  
- [ ] Macro-driven **validation DSL**  
- [ ] Schema **generation and introspection**
