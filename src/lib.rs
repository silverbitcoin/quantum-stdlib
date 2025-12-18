//! # Quantum Standard Library
//!
//! Standard library modules for Quantum smart contracts.
//!
//! This crate provides:
//! - Vector operations
//! - Option types
//! - Object manipulation utilities
//! - String operations
//! - Math utilities

#![warn(missing_docs, rust_2018_idioms)]
#![forbid(unsafe_code)]

/// Vector operations and utilities for dynamic arrays.
///
/// Provides vector manipulation functions including:
/// - Creation and initialization
/// - Element access and modification
/// - Iteration and transformation
/// - Capacity management
pub mod vector;

/// Option type for optional values.
///
/// Provides the Option type and utilities for:
/// - Representing Some(value) or None
/// - Safe value extraction
/// - Chaining operations
pub mod option;

/// Object manipulation and metadata utilities.
///
/// Provides utilities for:
/// - Object reference management
/// - Ownership tracking
/// - Metadata access and modification
pub mod object;

/// String operations and utilities.
///
/// Provides string manipulation functions including:
/// - String creation and concatenation
/// - Character operations
/// - String formatting
/// - Encoding/decoding
pub mod string;

/// Mathematical operations and utilities.
///
/// Provides mathematical functions including:
/// - Arithmetic operations
/// - Trigonometric functions
/// - Logarithmic functions
/// - Random number generation
pub mod math;

pub use math::*;
pub use object::{ObjectMetadata, ObjectRef, Owner};
pub use option::Option;
pub use string::String;
pub use vector::Vector;
