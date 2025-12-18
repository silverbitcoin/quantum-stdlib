//! # Vector Operations Module
//!
//! Provides vector (dynamic array) operations for Quantum smart contracts.
//! This is a PRODUCTION-READY implementation with:
//! - Type-safe vector operations
//! - Bounds checking
//! - Resource safety
//! - Efficient memory management

use serde::{Deserialize, Serialize};
use std::fmt;

/// Generic vector type for Quantum smart contracts
///
/// Vectors are dynamic arrays that can grow and shrink at runtime.
/// They enforce type safety and bounds checking.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Vector<T> {
    elements: Vec<T>,
}

impl<T> Vector<T> {
    /// Create a new empty vector
    ///
    /// # Examples
    ///
    /// ```
    /// use quantum_stdlib::Vector;
    ///
    /// let vec: Vector<u64> = Vector::new();
    /// assert_eq!(vec.len(), 0);
    /// assert!(vec.is_empty());
    /// ```
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
    }

    /// Create a new vector with specified capacity
    ///
    /// # Arguments
    ///
    /// * `capacity` - Initial capacity to allocate
    ///
    /// # Examples
    ///
    /// ```
    /// use quantum_stdlib::Vector;
    ///
    /// let vec: Vector<u64> = Vector::with_capacity(10);
    /// assert_eq!(vec.len(), 0);
    /// ```
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            elements: Vec::with_capacity(capacity),
        }
    }

    /// Get the number of elements in the vector
    ///
    /// # Examples
    ///
    /// ```
    /// use quantum_stdlib::Vector;
    ///
    /// let mut vec = Vector::new();
    /// vec.push(1u64);
    /// vec.push(2u64);
    /// assert_eq!(vec.len(), 2);
    /// ```
    pub fn len(&self) -> u64 {
        self.elements.len() as u64
    }

    /// Check if the vector is empty
    ///
    /// # Examples
    ///
    /// ```
    /// use quantum_stdlib::Vector;
    ///
    /// let vec: Vector<u64> = Vector::new();
    /// assert!(vec.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    /// Push an element to the end of the vector
    ///
    /// # Arguments
    ///
    /// * `value` - Element to push
    ///
    /// # Examples
    ///
    /// ```
    /// use quantum_stdlib::Vector;
    ///
    /// let mut vec = Vector::new();
    /// vec.push(42u64);
    /// assert_eq!(vec.len(), 1);
    /// ```
    pub fn push(&mut self, value: T) {
        self.elements.push(value);
    }

    /// Pop an element from the end of the vector
    ///
    /// # Returns
    ///
    /// * `Some(T)` - The last element if the vector is not empty
    /// * `None` - If the vector is empty
    ///
    /// # Examples
    ///
    /// ```
    /// use quantum_stdlib::Vector;
    ///
    /// let mut vec = Vector::new();
    /// vec.push(42u64);
    /// assert_eq!(vec.pop(), Some(42u64));
    /// assert_eq!(vec.pop(), None);
    /// ```
    pub fn pop(&mut self) -> Option<T> {
        self.elements.pop()
    }

    /// Get a reference to an element at the specified index
    ///
    /// # Arguments
    ///
    /// * `index` - Index of the element
    ///
    /// # Returns
    ///
    /// * `Some(&T)` - Reference to the element if index is valid
    /// * `None` - If index is out of bounds
    ///
    /// # Examples
    ///
    /// ```
    /// use quantum_stdlib::Vector;
    ///
    /// let mut vec = Vector::new();
    /// vec.push(42u64);
    /// assert_eq!(vec.get(0), Some(&42u64));
    /// assert_eq!(vec.get(1), None);
    /// ```
    pub fn get(&self, index: u64) -> Option<&T> {
        self.elements.get(index as usize)
    }

    /// Get a mutable reference to an element at the specified index
    ///
    /// # Arguments
    ///
    /// * `index` - Index of the element
    ///
    /// # Returns
    ///
    /// * `Some(&mut T)` - Mutable reference to the element if index is valid
    /// * `None` - If index is out of bounds
    ///
    /// # Examples
    ///
    /// ```
    /// use quantum_stdlib::Vector;
    ///
    /// let mut vec = Vector::new();
    /// vec.push(42u64);
    /// if let Some(elem) = vec.get_mut(0) {
    ///     *elem = 100;
    /// }
    /// assert_eq!(vec.get(0), Some(&100u64));
    /// ```
    pub fn get_mut(&mut self, index: u64) -> Option<&mut T> {
        self.elements.get_mut(index as usize)
    }

    /// Set the value at the specified index
    ///
    /// # Arguments
    ///
    /// * `index` - Index of the element
    /// * `value` - New value
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the index is valid
    /// * `Err(String)` - If the index is out of bounds
    ///
    /// # Examples
    ///
    /// ```
    /// use quantum_stdlib::Vector;
    ///
    /// let mut vec = Vector::new();
    /// vec.push(42u64);
    /// vec.set(0, 100u64).unwrap();
    /// assert_eq!(vec.get(0), Some(&100u64));
    /// ```
    pub fn set(&mut self, index: u64, value: T) -> Result<(), String> {
        let idx = index as usize;
        if idx < self.elements.len() {
            self.elements[idx] = value;
            Ok(())
        } else {
            Err(format!(
                "Index out of bounds: {} >= {}",
                index,
                self.elements.len()
            ))
        }
    }

    /// Swap two elements in the vector
    ///
    /// # Arguments
    ///
    /// * `i` - Index of first element
    /// * `j` - Index of second element
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If both indices are valid
    /// * `Err(String)` - If either index is out of bounds
    ///
    /// # Examples
    ///
    /// ```
    /// use quantum_stdlib::Vector;
    ///
    /// let mut vec = Vector::new();
    /// vec.push(1u64);
    /// vec.push(2u64);
    /// vec.swap(0, 1).unwrap();
    /// assert_eq!(vec.get(0), Some(&2u64));
    /// assert_eq!(vec.get(1), Some(&1u64));
    /// ```
    pub fn swap(&mut self, i: u64, j: u64) -> Result<(), String> {
        let i_usize = i as usize;
        let j_usize = j as usize;
        let len = self.elements.len();

        if i_usize >= len {
            return Err(format!("Index out of bounds: {} >= {}", i, len));
        }
        if j_usize >= len {
            return Err(format!("Index out of bounds: {} >= {}", j, len));
        }

        self.elements.swap(i_usize, j_usize);
        Ok(())
    }

    /// Remove an element at the specified index
    ///
    /// # Arguments
    ///
    /// * `index` - Index of the element to remove
    ///
    /// # Returns
    ///
    /// * `Ok(T)` - The removed element if index is valid
    /// * `Err(String)` - If index is out of bounds
    ///
    /// # Examples
    ///
    /// ```
    /// use quantum_stdlib::Vector;
    ///
    /// let mut vec = Vector::new();
    /// vec.push(1u64);
    /// vec.push(2u64);
    /// vec.push(3u64);
    /// assert_eq!(vec.remove(1).unwrap(), 2u64);
    /// assert_eq!(vec.len(), 2);
    /// ```
    pub fn remove(&mut self, index: u64) -> Result<T, String> {
        let idx = index as usize;
        if idx < self.elements.len() {
            Ok(self.elements.remove(idx))
        } else {
            Err(format!(
                "Index out of bounds: {} >= {}",
                index,
                self.elements.len()
            ))
        }
    }

    /// Insert an element at the specified index
    ///
    /// # Arguments
    ///
    /// * `index` - Index where to insert
    /// * `value` - Element to insert
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the index is valid (0 <= index <= len)
    /// * `Err(String)` - If the index is out of bounds
    ///
    /// # Examples
    ///
    /// ```
    /// use quantum_stdlib::Vector;
    ///
    /// let mut vec = Vector::new();
    /// vec.push(1u64);
    /// vec.push(3u64);
    /// vec.insert(1, 2u64).unwrap();
    /// assert_eq!(vec.get(1), Some(&2u64));
    /// ```
    pub fn insert(&mut self, index: u64, value: T) -> Result<(), String> {
        let idx = index as usize;
        if idx <= self.elements.len() {
            self.elements.insert(idx, value);
            Ok(())
        } else {
            Err(format!(
                "Index out of bounds: {} > {}",
                index,
                self.elements.len()
            ))
        }
    }

    /// Clear all elements from the vector
    ///
    /// # Examples
    ///
    /// ```
    /// use quantum_stdlib::Vector;
    ///
    /// let mut vec = Vector::new();
    /// vec.push(1u64);
    /// vec.push(2u64);
    /// vec.clear();
    /// assert!(vec.is_empty());
    /// ```
    pub fn clear(&mut self) {
        self.elements.clear();
    }

    /// Check if the vector contains a specific element
    ///
    /// # Arguments
    ///
    /// * `value` - Element to search for
    ///
    /// # Returns
    ///
    /// * `true` - If the element is found
    /// * `false` - If the element is not found
    ///
    /// # Examples
    ///
    /// ```
    /// use quantum_stdlib::Vector;
    ///
    /// let mut vec = Vector::new();
    /// vec.push(1u64);
    /// vec.push(2u64);
    /// assert!(vec.contains(&2u64));
    /// assert!(!vec.contains(&3u64));
    /// ```
    pub fn contains(&self, value: &T) -> bool
    where
        T: PartialEq,
    {
        self.elements.contains(value)
    }

    /// Reverse the order of elements in the vector
    ///
    /// # Examples
    ///
    /// ```
    /// use quantum_stdlib::Vector;
    ///
    /// let mut vec = Vector::new();
    /// vec.push(1u64);
    /// vec.push(2u64);
    /// vec.push(3u64);
    /// vec.reverse();
    /// assert_eq!(vec.get(0), Some(&3u64));
    /// assert_eq!(vec.get(1), Some(&2u64));
    /// assert_eq!(vec.get(2), Some(&1u64));
    /// ```
    pub fn reverse(&mut self) {
        self.elements.reverse();
    }

    /// Get an iterator over the vector elements
    ///
    /// # Examples
    ///
    /// ```
    /// use quantum_stdlib::Vector;
    ///
    /// let mut vec = Vector::new();
    /// vec.push(1u64);
    /// vec.push(2u64);
    /// vec.push(3u64);
    ///
    /// let sum: u64 = vec.iter().sum();
    /// assert_eq!(sum, 6);
    /// ```
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.elements.iter()
    }

    /// Get a mutable iterator over the vector elements
    ///
    /// # Examples
    ///
    /// ```
    /// use quantum_stdlib::Vector;
    ///
    /// let mut vec = Vector::new();
    /// vec.push(1u64);
    /// vec.push(2u64);
    ///
    /// for elem in vec.iter_mut() {
    ///     *elem *= 2;
    /// }
    ///
    /// assert_eq!(vec.get(0), Some(&2u64));
    /// assert_eq!(vec.get(1), Some(&4u64));
    /// ```
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.elements.iter_mut()
    }

    /// Convert the vector into its inner Vec
    ///
    /// # Examples
    ///
    /// ```
    /// use quantum_stdlib::Vector;
    ///
    /// let mut vec = Vector::new();
    /// vec.push(1u64);
    /// vec.push(2u64);
    ///
    /// let inner = vec.into_inner();
    /// assert_eq!(inner, vec![1u64, 2u64]);
    /// ```
    pub fn into_inner(self) -> Vec<T> {
        self.elements
    }

    /// Create a vector from a Vec
    ///
    /// # Arguments
    ///
    /// * `vec` - The Vec to convert
    ///
    /// # Examples
    ///
    /// ```
    /// use quantum_stdlib::Vector;
    ///
    /// let vec = Vector::from_vec(vec![1u64, 2u64, 3u64]);
    /// assert_eq!(vec.len(), 3);
    /// ```
    pub fn from_vec(vec: Vec<T>) -> Self {
        Self { elements: vec }
    }
}

impl<T> Default for Vector<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: fmt::Display> fmt::Display for Vector<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for (i, elem) in self.elements.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", elem)?;
        }
        write!(f, "]")
    }
}

impl<T> FromIterator<T> for Vector<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self {
            elements: iter.into_iter().collect(),
        }
    }
}
