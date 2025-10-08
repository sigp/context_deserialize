//! Context-aware deserialization for Rust data structures.
//!
//! This crate provides the `ContextDeserialize` trait, which extends serde's deserialization
//! capabilities by allowing you to pass additional context during deserialization.

mod impls;

#[cfg(feature = "derive")]
pub use context_deserialize_derive::context_deserialize;

use serde::de::Deserializer;

/// A trait for deserializing data structures with additional context.
///
/// This trait is similar to serde's `Deserialize` trait, but with an additional context parameter
/// of type `C` that can be passed through the deserialization process. This is useful when you need
/// external information to properly deserialize your data structures.
pub trait ContextDeserialize<'de, C>: Sized {
    /// Deserialize this value from the given serde deserializer with additional context.
    ///
    /// # Parameters
    ///
    /// - `deserializer`: The serde deserializer to read from
    /// - `context`: Additional context that can be used during deserialization
    ///
    /// # Errors
    ///
    /// Returns a deserialization error if the data cannot be deserialized into this type.
    fn context_deserialize<D>(deserializer: D, context: C) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>;
}
