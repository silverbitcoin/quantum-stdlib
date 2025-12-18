//! # Option Type Module
//!
//! Provides the Option type for representing optional values.
//! This is a PRODUCTION-READY implementation with:
//! - Safe value extraction
//! - Chaining operations
//! - Pattern matching support

use serde::{Deserialize, Serialize};
use std::fmt;

/// Option type for representing optional values
///
/// An Option is either Some(value) or None.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Option<T> {
    /// Some value
    Some(T),
    /// No value
    None,
}

impl<T> Option<T> {
    /// Check if the option contains a value
    ///
    /// # Examples
    ///
    /// ```
    /// use quantum_stdlib::Option;
    ///
    /// let x: Option<u64> = Option::Some(42);
    /// assert!(x.is_some());
    ///
    /// let y: Option<u64> = Option::None;
    /// assert!(!y.is_some());
    /// ```
    pub fn is_some(&self) -> bool {
        matches!(self, Option::Some(_))
    }

    /// Check if the option is None
    ///
    /// # Examples
    ///
    /// ```
    /// use quantum_stdlib::Option;
    ///
    /// let x: Option<u64> = Option::Some(42);
    /// assert!(!x.is_none());
    ///
    /// let y: Option<u64> = Option::None;
    /// assert!(y.is_none());
    /// ```
    pub fn is_none(&self) -> bool {
        matches!(self, Option::None)
    }

    /// Extract the value from Some, or return a default
    ///
    /// # Arguments
    ///
    /// * `default` - Default value if None
    ///
    /// # Examples
    ///
    /// ```
    /// use quantum_stdlib::Option;
    ///
    /// let x: Option<u64> = Option::Some(42);
    /// assert_eq!(x.unwrap_or(0), 42);
    ///
    /// let y: Option<u64> = Option::None;
    /// assert_eq!(y.unwrap_or(0), 0);
    /// ```
    pub fn unwrap_or(&self, default: T) -> T
    where
        T: Copy,
    {
        match self {
            Option::Some(v) => *v,
            Option::None => default,
        }
    }

    /// Extract the value from Some, or compute a default
    ///
    /// # Arguments
    ///
    /// * `f` - Function to compute default value
    ///
    /// # Examples
    ///
    /// ```
    /// use quantum_stdlib::Option;
    ///
    /// let x: Option<u64> = Option::Some(42);
    /// assert_eq!(x.unwrap_or_else(|| 0), 42);
    ///
    /// let y: Option<u64> = Option::None;
    /// assert_eq!(y.unwrap_or_else(|| 100), 100);
    /// ```
    pub fn unwrap_or_else<F>(&self, f: F) -> T
    where
        T: Copy,
        F: FnOnce() -> T,
    {
        match self {
            Option::Some(v) => *v,
            Option::None => f(),
        }
    }

    /// Extract the value from Some, panicking if None
    ///
    /// # Panics
    ///
    /// Panics if the option is None
    ///
    /// # Examples
    ///
    /// ```
    /// use quantum_stdlib::Option;
    ///
    /// let x: Option<u64> = Option::Some(42);
    /// assert_eq!(x.unwrap(), 42);
    /// ```
    pub fn unwrap(&self) -> T
    where
        T: Copy,
    {
        match self {
            Option::Some(v) => *v,
            Option::None => panic!("Called unwrap on None"),
        }
    }

    /// Apply a function to the value if Some
    ///
    /// # Arguments
    ///
    /// * `f` - Function to apply
    ///
    /// # Examples
    ///
    /// ```
    /// use quantum_stdlib::Option;
    ///
    /// let x: Option<u64> = Option::Some(42);
    /// let y = x.map(|v| v * 2);
    /// assert_eq!(y, Option::Some(84));
    ///
    /// let z: Option<u64> = Option::None;
    /// let w = z.map(|v| v * 2);
    /// assert_eq!(w, Option::None);
    /// ```
    pub fn map<U, F>(&self, f: F) -> Option<U>
    where
        T: Copy,
        F: FnOnce(T) -> U,
    {
        match self {
            Option::Some(v) => Option::Some(f(*v)),
            Option::None => Option::None,
        }
    }

    /// Apply a function that returns an Option
    ///
    /// # Arguments
    ///
    /// * `f` - Function to apply
    ///
    /// # Examples
    ///
    /// ```
    /// use quantum_stdlib::Option;
    ///
    /// let x: Option<u64> = Option::Some(42);
    /// let y = x.and_then(|v| Option::Some(v * 2));
    /// assert_eq!(y, Option::Some(84));
    /// ```
    pub fn and_then<U, F>(&self, f: F) -> Option<U>
    where
        T: Copy,
        F: FnOnce(T) -> Option<U>,
    {
        match self {
            Option::Some(v) => f(*v),
            Option::None => Option::None,
        }
    }

    /// Filter the value based on a predicate
    ///
    /// # Arguments
    ///
    /// * `predicate` - Predicate function
    ///
    /// # Examples
    ///
    /// ```
    /// use quantum_stdlib::Option;
    ///
    /// let x: Option<u64> = Option::Some(42);
    /// let y = x.filter(|v| v > &40);
    /// assert_eq!(y, Option::Some(42));
    ///
    /// let z = x.filter(|v| v > &50);
    /// assert_eq!(z, Option::None);
    /// ```
    pub fn filter<F>(&self, predicate: F) -> Option<T>
    where
        T: Copy,
        F: FnOnce(T) -> bool,
    {
        match self {
            Option::Some(v) => {
                if predicate(*v) {
                    Option::Some(*v)
                } else {
                    Option::None
                }
            }
            Option::None => Option::None,
        }
    }

    /// Get a reference to the value if Some
    ///
    /// # Examples
    ///
    /// ```
    /// use quantum_stdlib::Option;
    ///
    /// let x: Option<u64> = Option::Some(42);
    /// assert_eq!(x.as_ref(), Option::Some(&42));
    ///
    /// let y: Option<u64> = Option::None;
    /// assert_eq!(y.as_ref(), Option::None);
    /// ```
    pub fn as_ref(&self) -> Option<&T> {
        match self {
            Option::Some(v) => Option::Some(v),
            Option::None => Option::None,
        }
    }

    /// Get a mutable reference to the value if Some
    ///
    /// # Examples
    ///
    /// ```
    /// use quantum_stdlib::Option;
    ///
    /// let mut x: Option<u64> = Option::Some(42);
    /// if let Option::Some(v) = x.as_mut() {
    ///     *v = 100;
    /// }
    /// assert_eq!(x, Option::Some(100));
    /// ```
    pub fn as_mut(&mut self) -> Option<&mut T> {
        match self {
            Option::Some(v) => Option::Some(v),
            Option::None => Option::None,
        }
    }

    /// Convert Option<T> to Option<U> by consuming self
    ///
    /// # Examples
    ///
    /// ```
    /// use quantum_stdlib::Option;
    ///
    /// let x: Option<u64> = Option::Some(42);
    /// let y = x.map_or(0, |v| v * 2);
    /// assert_eq!(y, 84);
    ///
    /// let z: Option<u64> = Option::None;
    /// let w = z.map_or(0, |v| v * 2);
    /// assert_eq!(w, 0);
    /// ```
    pub fn map_or<U, F>(&self, default: U, f: F) -> U
    where
        T: Copy,
        F: FnOnce(T) -> U,
    {
        match self {
            Option::Some(v) => f(*v),
            Option::None => default,
        }
    }

    /// Combine two options
    ///
    /// # Arguments
    ///
    /// * `other` - Another option
    ///
    /// # Examples
    ///
    /// ```
    /// use quantum_stdlib::Option;
    ///
    /// let x: Option<u64> = Option::Some(42);
    /// let y: Option<u64> = Option::Some(100);
    /// let z = x.and(y);
    /// assert_eq!(z, Option::Some(100));
    /// ```
    pub fn and<U>(&self, other: Option<U>) -> Option<U>
    where
        T: Copy,
    {
        match self {
            Option::Some(_) => other,
            Option::None => Option::None,
        }
    }

    /// Return self if Some, otherwise return other
    ///
    /// # Arguments
    ///
    /// * `other` - Alternative option
    ///
    /// # Examples
    ///
    /// ```
    /// use quantum_stdlib::Option;
    ///
    /// let x: Option<u64> = Option::Some(42);
    /// let y: Option<u64> = Option::Some(100);
    /// let z = x.or(y);
    /// assert_eq!(z, Option::Some(42));
    ///
    /// let a: Option<u64> = Option::None;
    /// let b: Option<u64> = Option::Some(100);
    /// let c = a.or(b);
    /// assert_eq!(c, Option::Some(100));
    /// ```
    pub fn or(&self, other: Option<T>) -> Option<T>
    where
        T: Copy,
    {
        match self {
            Option::Some(v) => Option::Some(*v),
            Option::None => other,
        }
    }
}

impl<T: fmt::Display> fmt::Display for Option<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Option::Some(v) => write!(f, "Some({})", v),
            Option::None => write!(f, "None"),
        }
    }
}
