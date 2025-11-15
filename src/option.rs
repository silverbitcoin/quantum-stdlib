//! # Option Type Module
//!
//! Provides Option type for representing optional values in Quantum smart contracts.
//! This is a PRODUCTION-READY implementation with:
//! - Type-safe optional value handling
//! - Null-safety guarantees
//! - Functional programming patterns
//! - Resource safety

use serde::{Deserialize, Serialize};
use std::fmt;

/// Option type representing an optional value
///
/// An Option can be either:
/// - `Some(T)` - Contains a value of type T
/// - `None` - Contains no value
///
/// This is similar to Rust's Option but designed for Quantum smart contracts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Option<T> {
    /// Some value of type T
    Some(T),
    /// No value
    None,
}

impl<T> Option<T> {
    /// Returns `true` if the option is a `Some` value
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

    /// Returns `true` if the option is a `None` value
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

    /// Returns the contained `Some` value, consuming the `self` value
    ///
    /// # Panics
    ///
    /// Panics if the value is a `None` with a custom panic message
    ///
    /// # Examples
    ///
    /// ```
    /// use quantum_stdlib::Option;
    ///
    /// let x: Option<u64> = Option::Some(42);
    /// assert_eq!(x.unwrap(), 42);
    /// ```
    ///
    /// ```should_panic
    /// use quantum_stdlib::Option;
    ///
    /// let x: Option<u64> = Option::None;
    /// x.unwrap(); // panics
    /// ```
    pub fn unwrap(self) -> T {
        match self {
            Option::Some(val) => val,
            Option::None => panic!("called `Option::unwrap()` on a `None` value"),
        }
    }

    /// Returns the contained `Some` value or a provided default
    ///
    /// # Arguments
    ///
    /// * `default` - The default value to return if `None`
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
    pub fn unwrap_or(self, default: T) -> T {
        match self {
            Option::Some(val) => val,
            Option::None => default,
        }
    }

    /// Returns the contained `Some` value or computes it from a closure
    ///
    /// # Arguments
    ///
    /// * `f` - A closure that returns the default value
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
    /// assert_eq!(y.unwrap_or_else(|| 10), 10);
    /// ```
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T,
    {
        match self {
            Option::Some(val) => val,
            Option::None => f(),
        }
    }

    /// Maps an `Option<T>` to `Option<U>` by applying a function to the contained value
    ///
    /// # Arguments
    ///
    /// * `f` - A function to apply to the contained value
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
    pub fn map<U, F>(self, f: F) -> Option<U>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            Option::Some(val) => Option::Some(f(val)),
            Option::None => Option::None,
        }
    }

    /// Maps an `Option<T>` to `Option<U>` by applying a function that returns an Option
    ///
    /// # Arguments
    ///
    /// * `f` - A function that returns an Option
    ///
    /// # Examples
    ///
    /// ```
    /// use quantum_stdlib::Option;
    ///
    /// fn divide(x: u64, y: u64) -> Option<u64> {
    ///     if y == 0 {
    ///         Option::None
    ///     } else {
    ///         Option::Some(x / y)
    ///     }
    /// }
    ///
    /// let x: Option<u64> = Option::Some(10);
    /// let y = x.and_then(|v| divide(v, 2));
    /// assert_eq!(y, Option::Some(5));
    ///
    /// let z = x.and_then(|v| divide(v, 0));
    /// assert_eq!(z, Option::None);
    /// ```
    pub fn and_then<U, F>(self, f: F) -> Option<U>
    where
        F: FnOnce(T) -> Option<U>,
    {
        match self {
            Option::Some(val) => f(val),
            Option::None => Option::None,
        }
    }

    /// Returns `None` if the option is `None`, otherwise calls `predicate` with the wrapped value
    ///
    /// # Arguments
    ///
    /// * `predicate` - A function that returns true if the value should be kept
    ///
    /// # Examples
    ///
    /// ```
    /// use quantum_stdlib::Option;
    ///
    /// let x: Option<u64> = Option::Some(42);
    /// let y = x.filter(|v| *v > 40);
    /// assert_eq!(y, Option::Some(42));
    ///
    /// let z = x.filter(|v| *v > 50);
    /// assert_eq!(z, Option::None);
    /// ```
    pub fn filter<F>(self, predicate: F) -> Option<T>
    where
        F: FnOnce(&T) -> bool,
    {
        match self {
            Option::Some(val) if predicate(&val) => Option::Some(val),
            _ => Option::None,
        }
    }

    /// Returns the option if it contains a value, otherwise returns `optb`
    ///
    /// # Arguments
    ///
    /// * `optb` - The alternative option
    ///
    /// # Examples
    ///
    /// ```
    /// use quantum_stdlib::Option;
    ///
    /// let x: Option<u64> = Option::Some(42);
    /// let y: Option<u64> = Option::None;
    /// assert_eq!(x.or(y), Option::Some(42));
    ///
    /// let a: Option<u64> = Option::None;
    /// let b: Option<u64> = Option::Some(100);
    /// assert_eq!(a.or(b), Option::Some(100));
    /// ```
    pub fn or(self, optb: Option<T>) -> Option<T> {
        match self {
            Option::Some(_) => self,
            Option::None => optb,
        }
    }

    /// Returns the option if it contains a value, otherwise calls `f` and returns the result
    ///
    /// # Arguments
    ///
    /// * `f` - A closure that returns an alternative Option
    ///
    /// # Examples
    ///
    /// ```
    /// use quantum_stdlib::Option;
    ///
    /// let x: Option<u64> = Option::Some(42);
    /// assert_eq!(x.or_else(|| Option::Some(0)), Option::Some(42));
    ///
    /// let y: Option<u64> = Option::None;
    /// assert_eq!(y.or_else(|| Option::Some(100)), Option::Some(100));
    /// ```
    pub fn or_else<F>(self, f: F) -> Option<T>
    where
        F: FnOnce() -> Option<T>,
    {
        match self {
            Option::Some(_) => self,
            Option::None => f(),
        }
    }

    /// Converts from `Option<T>` to `Option<&T>`
    ///
    /// # Examples
    ///
    /// ```
    /// use quantum_stdlib::Option;
    ///
    /// let x: Option<u64> = Option::Some(42);
    /// let y: Option<&u64> = x.as_ref();
    /// assert_eq!(y, Option::Some(&42));
    /// ```
    pub fn as_ref(&self) -> Option<&T> {
        match self {
            Option::Some(ref val) => Option::Some(val),
            Option::None => Option::None,
        }
    }

    /// Converts from `Option<T>` to `Option<&mut T>`
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
            Option::Some(ref mut val) => Option::Some(val),
            Option::None => Option::None,
        }
    }

    /// Takes the value out of the option, leaving a `None` in its place
    ///
    /// # Examples
    ///
    /// ```
    /// use quantum_stdlib::Option;
    ///
    /// let mut x: Option<u64> = Option::Some(42);
    /// let y = x.take();
    /// assert_eq!(y, Option::Some(42));
    /// assert_eq!(x, Option::None);
    /// ```
    pub fn take(&mut self) -> Option<T> {
        std::mem::replace(self, Option::None)
    }

    /// Replaces the actual value in the option by the value given in parameter,
    /// returning the old value if present
    ///
    /// # Arguments
    ///
    /// * `value` - The new value
    ///
    /// # Examples
    ///
    /// ```
    /// use quantum_stdlib::Option;
    ///
    /// let mut x: Option<u64> = Option::Some(42);
    /// let old = x.replace(100);
    /// assert_eq!(old, Option::Some(42));
    /// assert_eq!(x, Option::Some(100));
    /// ```
    pub fn replace(&mut self, value: T) -> Option<T> {
        std::mem::replace(self, Option::Some(value))
    }
}

impl<T: Default> Option<T> {
    /// Returns the contained `Some` value or a default
    ///
    /// # Examples
    ///
    /// ```
    /// use quantum_stdlib::Option;
    ///
    /// let x: Option<u64> = Option::Some(42);
    /// assert_eq!(x.unwrap_or_default(), 42);
    ///
    /// let y: Option<u64> = Option::None;
    /// assert_eq!(y.unwrap_or_default(), 0);
    /// ```
    pub fn unwrap_or_default(self) -> T {
        match self {
            Option::Some(val) => val,
            Option::None => T::default(),
        }
    }
}

impl<T> Default for Option<T> {
    fn default() -> Self {
        Option::None
    }
}

impl<T: fmt::Display> fmt::Display for Option<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Option::Some(val) => write!(f, "Some({})", val),
            Option::None => write!(f, "None"),
        }
    }
}

impl<T> From<std::option::Option<T>> for Option<T> {
    fn from(opt: std::option::Option<T>) -> Self {
        match opt {
            std::option::Option::Some(val) => Option::Some(val),
            std::option::Option::None => Option::None,
        }
    }
}

impl<T> From<Option<T>> for std::option::Option<T> {
    fn from(opt: Option<T>) -> Self {
        match opt {
            Option::Some(val) => std::option::Option::Some(val),
            Option::None => std::option::Option::None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_some_is_none() {
        let x: Option<u64> = Option::Some(42);
        assert!(x.is_some());
        assert!(!x.is_none());

        let y: Option<u64> = Option::None;
        assert!(!y.is_some());
        assert!(y.is_none());
    }

    #[test]
    fn test_unwrap() {
        let x: Option<u64> = Option::Some(42);
        assert_eq!(x.unwrap(), 42);
    }

    #[test]
    #[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
    fn test_unwrap_panic() {
        let x: Option<u64> = Option::None;
        x.unwrap();
    }

    #[test]
    fn test_unwrap_or() {
        let x: Option<u64> = Option::Some(42);
        assert_eq!(x.unwrap_or(0), 42);

        let y: Option<u64> = Option::None;
        assert_eq!(y.unwrap_or(0), 0);
    }

    #[test]
    fn test_unwrap_or_else() {
        let x: Option<u64> = Option::Some(42);
        assert_eq!(x.unwrap_or_else(|| 0), 42);

        let y: Option<u64> = Option::None;
        assert_eq!(y.unwrap_or_else(|| 10), 10);
    }

    #[test]
    fn test_map() {
        let x: Option<u64> = Option::Some(42);
        let y = x.map(|v| v * 2);
        assert_eq!(y, Option::Some(84));

        let z: Option<u64> = Option::None;
        let w = z.map(|v| v * 2);
        assert_eq!(w, Option::None);
    }

    #[test]
    fn test_and_then() {
        fn divide(x: u64, y: u64) -> Option<u64> {
            if y == 0 {
                Option::None
            } else {
                Option::Some(x / y)
            }
        }

        let x: Option<u64> = Option::Some(10);
        let y = x.and_then(|v| divide(v, 2));
        assert_eq!(y, Option::Some(5));

        let z = x.and_then(|v| divide(v, 0));
        assert_eq!(z, Option::None);
    }

    #[test]
    fn test_filter() {
        let x: Option<u64> = Option::Some(42);
        let y = x.filter(|v| *v > 40);
        assert_eq!(y, Option::Some(42));

        let z = x.filter(|v| *v > 50);
        assert_eq!(z, Option::None);
    }

    #[test]
    fn test_or() {
        let x: Option<u64> = Option::Some(42);
        let y: Option<u64> = Option::None;
        assert_eq!(x.or(y), Option::Some(42));

        let a: Option<u64> = Option::None;
        let b: Option<u64> = Option::Some(100);
        assert_eq!(a.or(b), Option::Some(100));
    }

    #[test]
    fn test_take() {
        let mut x: Option<u64> = Option::Some(42);
        let y = x.take();
        assert_eq!(y, Option::Some(42));
        assert_eq!(x, Option::None);
    }

    #[test]
    fn test_replace() {
        let mut x: Option<u64> = Option::Some(42);
        let old = x.replace(100);
        assert_eq!(old, Option::Some(42));
        assert_eq!(x, Option::Some(100));
    }

    #[test]
    fn test_unwrap_or_default() {
        let x: Option<u64> = Option::Some(42);
        assert_eq!(x.unwrap_or_default(), 42);

        let y: Option<u64> = Option::None;
        assert_eq!(y.unwrap_or_default(), 0);
    }

    #[test]
    fn test_conversion_from_std() {
        let std_some = std::option::Option::Some(42u64);
        let quantum_some: Option<u64> = std_some.into();
        assert_eq!(quantum_some, Option::Some(42));

        let std_none: std::option::Option<u64> = std::option::Option::None;
        let quantum_none: Option<u64> = std_none.into();
        assert_eq!(quantum_none, Option::None);
    }
}
