//! # String Operations Module
//!
//! Provides string manipulation functions for Quantum smart contracts.
//! This is a PRODUCTION-READY implementation with:
//! - String creation and concatenation
//! - Character operations
//! - String formatting
//! - Encoding/decoding

use serde::{Deserialize, Serialize};
use std::fmt;

/// String type for Quantum smart contracts
///
/// Strings are UTF-8 encoded byte sequences.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct String {
    bytes: Vec<u8>,
}

impl String {
    /// Create a new empty string
    ///
    /// # Examples
    ///
    /// ```
    /// use quantum_stdlib::String;
    ///
    /// let s = String::new();
    /// assert!(s.is_empty());
    /// ```
    pub fn new() -> Self {
        Self { bytes: Vec::new() }
    }

    /// Create a string from a byte slice
    ///
    /// # Arguments
    ///
    /// * `bytes` - UTF-8 encoded bytes
    ///
    /// # Returns
    ///
    /// * `Ok(String)` - If bytes are valid UTF-8
    /// * `Err(String)` - If bytes are not valid UTF-8
    ///
    /// # Examples
    ///
    /// ```
    /// use quantum_stdlib::String;
    ///
    /// let s = String::from_bytes(b"hello").unwrap();
    /// assert_eq!(s.len(), 5);
    /// ```
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, std::string::String> {
        // Validate UTF-8
        std::str::from_utf8(bytes).map_err(|_| "Invalid UTF-8".to_string())?;

        Ok(Self {
            bytes: bytes.to_vec(),
        })
    }

    /// Create a string from a Rust string literal
    ///
    /// # Arguments
    ///
    /// * `s` - Rust string slice
    ///
    /// # Examples
    ///
    /// ```
    /// use quantum_stdlib::String;
    ///
    /// let s = String::from_str("hello");
    /// assert_eq!(s.len(), 5);
    /// ```
    pub fn from_str(s: &str) -> Self {
        Self {
            bytes: s.as_bytes().to_vec(),
        }
    }

    /// Get the length of the string in bytes
    ///
    /// # Examples
    ///
    /// ```
    /// use quantum_stdlib::String;
    ///
    /// let s = String::from_str("hello");
    /// assert_eq!(s.len(), 5);
    /// ```
    pub fn len(&self) -> u64 {
        self.bytes.len() as u64
    }

    /// Check if the string is empty
    ///
    /// # Examples
    ///
    /// ```
    /// use quantum_stdlib::String;
    ///
    /// let s = String::new();
    /// assert!(s.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    /// Get the bytes of the string
    ///
    /// # Examples
    ///
    /// ```
    /// use quantum_stdlib::String;
    ///
    /// let s = String::from_str("hello");
    /// assert_eq!(s.as_bytes(), b"hello");
    /// ```
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Convert to a Rust string slice
    ///
    /// # Returns
    ///
    /// * `Ok(&str)` - If the string is valid UTF-8
    /// * `Err(String)` - If the string is not valid UTF-8
    ///
    /// # Examples
    ///
    /// ```
    /// use quantum_stdlib::String;
    ///
    /// let s = String::from_str("hello");
    /// assert_eq!(s.as_str().unwrap(), "hello");
    /// ```
    pub fn as_str(&self) -> Result<&str, std::string::String> {
        std::str::from_utf8(&self.bytes).map_err(|_| "Invalid UTF-8".to_string())
    }

    /// Append another string
    ///
    /// # Arguments
    ///
    /// * `other` - String to append
    ///
    /// # Examples
    ///
    /// ```
    /// use quantum_stdlib::String;
    ///
    /// let mut s = String::from_str("hello");
    /// s.push_str(&String::from_str(" world"));
    /// assert_eq!(s.as_str().unwrap(), "hello world");
    /// ```
    pub fn push_str(&mut self, other: &String) {
        self.bytes.extend_from_slice(&other.bytes);
    }

    /// Append a single character
    ///
    /// # Arguments
    ///
    /// * `ch` - Character to append
    ///
    /// # Examples
    ///
    /// ```
    /// use quantum_stdlib::String;
    ///
    /// let mut s = String::from_str("hello");
    /// s.push_char('!');
    /// assert_eq!(s.as_str().unwrap(), "hello!");
    /// ```
    pub fn push_char(&mut self, ch: char) {
        let mut buf = [0; 4];
        let encoded = ch.encode_utf8(&mut buf);
        self.bytes.extend_from_slice(encoded.as_bytes());
    }

    /// Check if the string contains a substring
    ///
    /// # Arguments
    ///
    /// * `needle` - Substring to search for
    ///
    /// # Returns
    ///
    /// true if the substring is found, false otherwise
    ///
    /// # Examples
    ///
    /// ```
    /// use quantum_stdlib::String;
    ///
    /// let s = String::from_str("hello world");
    /// assert!(s.contains(&String::from_str("world")));
    /// assert!(!s.contains(&String::from_str("foo")));
    /// ```
    pub fn contains(&self, needle: &String) -> bool {
        self.bytes
            .windows(needle.bytes.len())
            .any(|window| window == needle.bytes.as_slice())
    }

    /// Find the index of a substring
    ///
    /// # Arguments
    ///
    /// * `needle` - Substring to search for
    ///
    /// # Returns
    ///
    /// * `Some(index)` - If the substring is found
    /// * `None` - If the substring is not found
    ///
    /// # Examples
    ///
    /// ```
    /// use quantum_stdlib::String;
    ///
    /// let s = String::from_str("hello world");
    /// assert_eq!(s.find(&String::from_str("world")), Some(6));
    /// assert_eq!(s.find(&String::from_str("foo")), None);
    /// ```
    pub fn find(&self, needle: &String) -> Option<u64> {
        self.bytes
            .windows(needle.bytes.len())
            .position(|window| window == needle.bytes.as_slice())
            .map(|pos| pos as u64)
    }

    /// Get a substring
    ///
    /// # Arguments
    ///
    /// * `start` - Start index
    /// * `end` - End index (exclusive)
    ///
    /// # Returns
    ///
    /// * `Ok(String)` - The substring
    /// * `Err(String)` - If indices are out of bounds
    ///
    /// # Examples
    ///
    /// ```
    /// use quantum_stdlib::String;
    ///
    /// let s = String::from_str("hello");
    /// let sub = s.substring(1, 4).unwrap();
    /// assert_eq!(sub.as_str().unwrap(), "ell");
    /// ```
    pub fn substring(&self, start: u64, end: u64) -> Result<String, std::string::String> {
        let start = start as usize;
        let end = end as usize;

        if start > self.bytes.len() || end > self.bytes.len() || start > end {
            return Err("Invalid substring indices".to_string());
        }

        Ok(String {
            bytes: self.bytes[start..end].to_vec(),
        })
    }

    /// Convert to uppercase
    ///
    /// # Examples
    ///
    /// ```
    /// use quantum_stdlib::String;
    ///
    /// let s = String::from_str("hello");
    /// let upper = s.to_uppercase();
    /// assert_eq!(upper.as_str().unwrap(), "HELLO");
    /// ```
    pub fn to_uppercase(&self) -> String {
        if let Ok(s) = self.as_str() {
            String::from_str(&s.to_uppercase())
        } else {
            self.clone()
        }
    }

    /// Convert to lowercase
    ///
    /// # Examples
    ///
    /// ```
    /// use quantum_stdlib::String;
    ///
    /// let s = String::from_str("HELLO");
    /// let lower = s.to_lowercase();
    /// assert_eq!(lower.as_str().unwrap(), "hello");
    /// ```
    pub fn to_lowercase(&self) -> String {
        if let Ok(s) = self.as_str() {
            String::from_str(&s.to_lowercase())
        } else {
            self.clone()
        }
    }

    /// Trim whitespace from both ends
    ///
    /// # Examples
    ///
    /// ```
    /// use quantum_stdlib::String;
    ///
    /// let s = String::from_str("  hello  ");
    /// let trimmed = s.trim();
    /// assert_eq!(trimmed.as_str().unwrap(), "hello");
    /// ```
    pub fn trim(&self) -> String {
        if let Ok(s) = self.as_str() {
            String::from_str(s.trim())
        } else {
            self.clone()
        }
    }

    /// Split the string by a delimiter
    ///
    /// # Arguments
    ///
    /// * `delimiter` - Delimiter string
    ///
    /// # Returns
    ///
    /// Vector of substrings
    ///
    /// # Examples
    ///
    /// ```
    /// use quantum_stdlib::String;
    ///
    /// let s = String::from_str("hello,world,foo");
    /// let parts = s.split(&String::from_str(","));
    /// assert_eq!(parts.len(), 3);
    /// ```
    pub fn split(&self, delimiter: &String) -> Vec<String> {
        if delimiter.is_empty() {
            return vec![self.clone()];
        }

        let mut result = Vec::new();
        let mut start = 0;

        while let Some(pos) = self
            .substring(start as u64, self.bytes.len() as u64)
            .ok()
            .and_then(|s| s.find(delimiter))
        {
            let pos = pos as usize + start;
            if let Ok(part) = self.substring(start as u64, pos as u64) {
                result.push(part);
            }
            start = pos + delimiter.bytes.len();
        }

        if start <= self.bytes.len() {
            if let Ok(part) = self.substring(start as u64, self.bytes.len() as u64) {
                result.push(part);
            }
        }

        result
    }

    /// Repeat the string n times
    ///
    /// # Arguments
    ///
    /// * `n` - Number of repetitions
    ///
    /// # Examples
    ///
    /// ```
    /// use quantum_stdlib::String;
    ///
    /// let s = String::from_str("ab");
    /// let repeated = s.repeat(3);
    /// assert_eq!(repeated.as_str().unwrap(), "ababab");
    /// ```
    pub fn repeat(&self, n: u64) -> String {
        let mut result = String::new();
        for _ in 0..n {
            result.push_str(self);
        }
        result
    }

    /// Replace all occurrences of a substring
    ///
    /// # Arguments
    ///
    /// * `from` - Substring to replace
    /// * `to` - Replacement string
    ///
    /// # Examples
    ///
    /// ```
    /// use quantum_stdlib::String;
    ///
    /// let s = String::from_str("hello world");
    /// let replaced = s.replace(&String::from_str("world"), &String::from_str("Quantum"));
    /// assert_eq!(replaced.as_str().unwrap(), "hello Quantum");
    /// ```
    pub fn replace(&self, from: &String, to: &String) -> String {
        let parts = self.split(from);
        let mut result = String::new();

        for (i, part) in parts.iter().enumerate() {
            if i > 0 {
                result.push_str(to);
            }
            result.push_str(part);
        }

        result
    }

    /// Check if the string starts with a prefix
    ///
    /// # Arguments
    ///
    /// * `prefix` - Prefix to check
    ///
    /// # Examples
    ///
    /// ```
    /// use quantum_stdlib::String;
    ///
    /// let s = String::from_str("hello world");
    /// assert!(s.starts_with(&String::from_str("hello")));
    /// assert!(!s.starts_with(&String::from_str("world")));
    /// ```
    pub fn starts_with(&self, prefix: &String) -> bool {
        self.bytes.starts_with(&prefix.bytes)
    }

    /// Check if the string ends with a suffix
    ///
    /// # Arguments
    ///
    /// * `suffix` - Suffix to check
    ///
    /// # Examples
    ///
    /// ```
    /// use quantum_stdlib::String;
    ///
    /// let s = String::from_str("hello world");
    /// assert!(s.ends_with(&String::from_str("world")));
    /// assert!(!s.ends_with(&String::from_str("hello")));
    /// ```
    pub fn ends_with(&self, suffix: &String) -> bool {
        self.bytes.ends_with(&suffix.bytes)
    }
}

impl Default for String {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for String {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Ok(s) = self.as_str() {
            write!(f, "{}", s)
        } else {
            write!(f, "<invalid UTF-8>")
        }
    }
}
