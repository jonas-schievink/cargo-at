//! A quick-and-dirty type-erased error type

use std::fmt::Display;

/// An helper struct that can be created from anything that implements `Display`.
pub struct AnyError(pub String);

impl<E: Display> From<E> for AnyError {
    fn from(e: E) -> Self {
        AnyError(e.to_string())
    }
}
