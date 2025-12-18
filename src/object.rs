//! # Object Operations Module
//!
//! Provides object reference management and metadata utilities.
//! This is a PRODUCTION-READY implementation with:
//! - Object reference management
//! - Ownership tracking
//! - Metadata access and modification

use serde::{Deserialize, Serialize};
use silver_core::ObjectID;
use std::fmt;

/// Object reference for accessing objects in storage
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ObjectRef {
    /// The object ID
    pub id: ObjectID,
    /// The version of the object
    pub version: u64,
    /// The digest of the object
    pub digest: [u8; 32],
}

impl ObjectRef {
    /// Create a new object reference
    ///
    /// # Arguments
    ///
    /// * `id` - Object ID
    /// * `version` - Object version
    /// * `digest` - Object digest
    ///
    /// # Examples
    ///
    /// ```
    /// use quantum_stdlib::ObjectRef;
    /// use silver_core::ObjectID;
    ///
    /// let id = ObjectID::new([0u8; 64]);
    /// let obj_ref = ObjectRef::new(id, 1, [0u8; 32]);
    /// assert_eq!(obj_ref.version, 1);
    /// ```
    pub fn new(id: ObjectID, version: u64, digest: [u8; 32]) -> Self {
        Self {
            id,
            version,
            digest,
        }
    }

    /// Get the object ID
    pub fn id(&self) -> ObjectID {
        self.id
    }

    /// Get the object version
    pub fn version(&self) -> u64 {
        self.version
    }

    /// Get the object digest
    pub fn digest(&self) -> &[u8; 32] {
        &self.digest
    }
}

impl fmt::Display for ObjectRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ObjectRef(id: {}, version: {})", self.id, self.version)
    }
}

/// Ownership information for an object
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Owner {
    /// Object is owned by an address
    Address(silver_core::SilverAddress),
    /// Object is shared (no single owner)
    Shared,
    /// Object is immutable (frozen)
    Immutable,
}

impl Owner {
    /// Check if the owner is an address
    ///
    /// # Examples
    ///
    /// ```
    /// use quantum_stdlib::Owner;
    /// use silver_core::SilverAddress;
    ///
    /// let addr = SilverAddress([0u8; 64]);
    /// let owner = Owner::Address(addr);
    /// assert!(owner.is_address());
    /// ```
    pub fn is_address(&self) -> bool {
        matches!(self, Owner::Address(_))
    }

    /// Check if the object is shared
    ///
    /// # Examples
    ///
    /// ```
    /// use quantum_stdlib::Owner;
    ///
    /// let owner = Owner::Shared;
    /// assert!(owner.is_shared());
    /// ```
    pub fn is_shared(&self) -> bool {
        matches!(self, Owner::Shared)
    }

    /// Check if the object is immutable
    ///
    /// # Examples
    ///
    /// ```
    /// use quantum_stdlib::Owner;
    ///
    /// let owner = Owner::Immutable;
    /// assert!(owner.is_immutable());
    /// ```
    pub fn is_immutable(&self) -> bool {
        matches!(self, Owner::Immutable)
    }

    /// Get the address if this is an address owner
    ///
    /// # Examples
    ///
    /// ```
    /// use quantum_stdlib::Owner;
    /// use silver_core::SilverAddress;
    ///
    /// let addr = SilverAddress([0u8; 64]);
    /// let owner = Owner::Address(addr);
    /// assert_eq!(owner.as_address(), Some(addr));
    /// ```
    pub fn as_address(&self) -> Option<silver_core::SilverAddress> {
        match self {
            Owner::Address(addr) => Some(*addr),
            _ => None,
        }
    }
}

impl fmt::Display for Owner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Owner::Address(addr) => write!(f, "Address({})", addr),
            Owner::Shared => write!(f, "Shared"),
            Owner::Immutable => write!(f, "Immutable"),
        }
    }
}

/// Object metadata
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ObjectMetadata {
    /// Object ID
    pub id: ObjectID,
    /// Object owner
    pub owner: Owner,
    /// Object version
    pub version: u64,
    /// Object size in bytes
    pub size: u64,
    /// Creation timestamp
    pub created_at: u64,
    /// Last modified timestamp
    pub modified_at: u64,
    /// Custom metadata (key-value pairs)
    pub custom: std::collections::HashMap<String, Vec<u8>>,
}

impl ObjectMetadata {
    /// Create new object metadata
    ///
    /// # Arguments
    ///
    /// * `id` - Object ID
    /// * `owner` - Object owner
    /// * `size` - Object size
    /// * `created_at` - Creation timestamp
    ///
    /// # Examples
    ///
    /// ```
    /// use quantum_stdlib::{ObjectMetadata, Owner};
    /// use silver_core::ObjectID;
    ///
    /// let id = ObjectID::new([0u8; 64]);
    /// let owner = Owner::Shared;
    /// let metadata = ObjectMetadata::new(id, owner, 1024, 1000);
    /// assert_eq!(metadata.size, 1024);
    /// ```
    pub fn new(id: ObjectID, owner: Owner, size: u64, created_at: u64) -> Self {
        Self {
            id,
            owner,
            version: 1,
            size,
            created_at,
            modified_at: created_at,
            custom: std::collections::HashMap::new(),
        }
    }

    /// Get the object ID
    pub fn id(&self) -> ObjectID {
        self.id
    }

    /// Get the object owner
    pub fn owner(&self) -> Owner {
        self.owner
    }

    /// Set the object owner
    ///
    /// # Arguments
    ///
    /// * `owner` - New owner
    pub fn set_owner(&mut self, owner: Owner) {
        self.owner = owner;
    }

    /// Get the object version
    pub fn version(&self) -> u64 {
        self.version
    }

    /// Increment the object version
    pub fn increment_version(&mut self) {
        self.version += 1;
    }

    /// Get the object size
    pub fn size(&self) -> u64 {
        self.size
    }

    /// Set the object size
    ///
    /// # Arguments
    ///
    /// * `size` - New size
    pub fn set_size(&mut self, size: u64) {
        self.size = size;
    }

    /// Get the creation timestamp
    pub fn created_at(&self) -> u64 {
        self.created_at
    }

    /// Get the last modified timestamp
    pub fn modified_at(&self) -> u64 {
        self.modified_at
    }

    /// Update the modified timestamp
    ///
    /// # Arguments
    ///
    /// * `timestamp` - New timestamp
    pub fn set_modified_at(&mut self, timestamp: u64) {
        self.modified_at = timestamp;
    }

    /// Set custom metadata
    ///
    /// # Arguments
    ///
    /// * `key` - Metadata key
    /// * `value` - Metadata value
    ///
    /// # Examples
    ///
    /// ```
    /// use quantum_stdlib::{ObjectMetadata, Owner};
    /// use silver_core::ObjectID;
    ///
    /// let id = ObjectID::new([0u8; 64]);
    /// let owner = Owner::Shared;
    /// let mut metadata = ObjectMetadata::new(id, owner, 1024, 1000);
    /// metadata.set_custom("type", b"coin");
    /// assert_eq!(metadata.get_custom("type"), Some(b"coin".to_vec()));
    /// ```
    pub fn set_custom(&mut self, key: &str, value: &[u8]) {
        self.custom.insert(key.to_string(), value.to_vec());
    }

    /// Get custom metadata
    ///
    /// # Arguments
    ///
    /// * `key` - Metadata key
    ///
    /// # Returns
    ///
    /// * `Some(value)` - If the key exists
    /// * `None` - If the key doesn't exist
    pub fn get_custom(&self, key: &str) -> Option<Vec<u8>> {
        self.custom.get(key).cloned()
    }

    /// Remove custom metadata
    ///
    /// # Arguments
    ///
    /// * `key` - Metadata key
    ///
    /// # Returns
    ///
    /// * `Some(value)` - If the key existed
    /// * `None` - If the key didn't exist
    pub fn remove_custom(&mut self, key: &str) -> Option<Vec<u8>> {
        self.custom.remove(key)
    }

    /// Check if custom metadata exists
    ///
    /// # Arguments
    ///
    /// * `key` - Metadata key
    ///
    /// # Returns
    ///
    /// true if the key exists, false otherwise
    pub fn has_custom(&self, key: &str) -> bool {
        self.custom.contains_key(key)
    }

    /// Get all custom metadata keys
    ///
    /// # Returns
    ///
    /// Vector of all custom metadata keys
    pub fn custom_keys(&self) -> Vec<String> {
        self.custom.keys().cloned().collect()
    }

    /// Clear all custom metadata
    pub fn clear_custom(&mut self) {
        self.custom.clear();
    }

    /// Get the age of the object in seconds
    ///
    /// # Arguments
    ///
    /// * `current_time` - Current timestamp
    ///
    /// # Returns
    ///
    /// Age in seconds
    pub fn age(&self, current_time: u64) -> u64 {
        if current_time > self.created_at {
            current_time - self.created_at
        } else {
            0
        }
    }

    /// Check if the object has been modified since creation
    ///
    /// # Returns
    ///
    /// true if modified, false otherwise
    pub fn is_modified(&self) -> bool {
        self.modified_at > self.created_at
    }

    /// Get the time since last modification
    ///
    /// # Arguments
    ///
    /// * `current_time` - Current timestamp
    ///
    /// # Returns
    ///
    /// Time since modification in seconds
    pub fn time_since_modified(&self, current_time: u64) -> u64 {
        if current_time > self.modified_at {
            current_time - self.modified_at
        } else {
            0
        }
    }
}

impl fmt::Display for ObjectMetadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ObjectMetadata {{ id: {}, owner: {}, version: {}, size: {} }}",
            self.id, self.owner, self.version, self.size
        )
    }
}
