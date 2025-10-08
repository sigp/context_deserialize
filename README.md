# context_deserialize

[![CI](https://github.com/sigp/context_deserialize/workflows/test-suite/badge.svg)](https://github.com/sigp/context_deserialize/actions)
[![Crates.io](https://img.shields.io/crates/v/context_deserialize.svg)](https://crates.io/crates/context_deserialize)
[![Documentation](https://docs.rs/context_deserialize/badge.svg)](https://docs.rs/context_deserialize)

A Rust library for deserializing data structures with additional context, built on top of [serde](https://serde.rs/).

## Overview

`context_deserialize` provides a trait similar to `serde::Deserialize`, but with an additional context parameter that can be passed through the deserialization process.
This is useful when you need external information (like a version number or configuration) to properly deserialize your data structures.
