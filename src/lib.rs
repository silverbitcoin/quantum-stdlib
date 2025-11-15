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

pub mod vector;
pub mod option;
pub mod object;
pub mod string;
pub mod math;

pub use vector::Vector;
pub use option::Option;
pub use object::{ObjectRef, Owner, ObjectMetadata};
