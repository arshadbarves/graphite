//! # Graphite
//!
//! A powerful, production-grade multi-agent framework for Rust.
//!
//! This is the main crate that re-exports functionality from all other
//! Graphite crates, providing a unified API for users.

#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(missing_docs)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

// Re-export core functionality (always available)
pub use graphite_core::*;

// Re-export optional features

/// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");