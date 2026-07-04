//! Domain error for the links feature. Krocy uses Kotlin `Result<Throwable>`;
//! Rust needs a concrete error carried through `Result<T, LinkError>`.

use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LinkError {
    /// Transport failure (request failed, non-2xx status).
    Network(String),
    /// Response body could not be decoded into the expected JSON shape.
    Decode(String),
}

impl fmt::Display for LinkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LinkError::Network(msg) => write!(f, "network error: {msg}"),
            LinkError::Decode(msg) => write!(f, "decode error: {msg}"),
        }
    }
}

impl std::error::Error for LinkError {}
