//! # Object Manipulation Utilities
//!
//! Provides utilities for working with blockchain objects in Quantum smart contracts.
//! This is a PRODUCTION-READY implementation with:
//! - Type-safe object operations
//! - Ownership management
//! - Object lifecycle utilities
//! - Resource safety

use serde::{Deserialize, Serialize};
use silver_core::{ObjectID, SilverAddress};
use std::fmt;

/// Object reference containing ID and version
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ObjectRef {
    /// Object ID (512-bit)
    pub id: ObjectID,
    /// Object version (sequence number)
    pub version: u64,
}

impl ObjectRef {
    /// Create a new object reference
    ///
    /// # Arguments
    ///
    /// * `id` - Object ID
    /// * `version` - Object version
    ///
    /// # Examples
    ///
    /// ```
    /// use quantum_stdlib::object::ObjectRef;
    /// use silver_core::ObjectID;
    ///
    /// let id = ObjectID::new([0u8; 64]);
    /// let obj_ref = ObjectRef::new(id, 1);
    /// assert_eq!(obj_ref.version, 1);
    /// ```
    pub fn new(id: ObjectID, version: u64) -> Self {
        Self { id, version }
    }

    /// Get the object ID
    ///
    /// # Examples
    ///
    /// ```
    /// use quantum_stdlib::object::ObjectRef;
    /// use silver_core::ObjectID;
    ///
    /// let id = ObjectID::new([0u8; 64]);
    /// let obj_ref = ObjectRef::new(id, 1);
    /// assert_eq!(obj_ref.id(), &id);
    /// ```
    pub fn id(&self) -> &ObjectID {
        &self.id
    }

    /// Get the object version
    ///
    /// # Examples
    ///
    /// ```
    /// use quantum_stdlib::object::ObjectRef;
    /// use silver_core::ObjectID;
    ///
    /// let id = ObjectID::new([0u8; 64]);
    /// let obj_ref = ObjectRef::new(id, 1);
    /// assert_eq!(obj_ref.version(), 1);
    /// ```
    pub fn version(&self) -> u64 {
        self.version
    }

    /// Increment the version number
    ///
    /// # Examples
    ///
    /// ```
    /// use quantum_stdlib::object::ObjectRef;
    /// use silver_core::ObjectID;
    ///
    /// let id = ObjectID::new([0u8; 64]);
    /// let mut obj_ref = ObjectRef::new(id, 1);
    /// obj_ref.increment_version();
    /// assert_eq!(obj_ref.version(), 2);
    /// ```
    pub fn increment_version(&mut self) {
        self.version += 1;
    }

    /// Create a new reference with incremented version
    ///
    /// # Examples
    ///
    /// ```
    /// use quantum_stdlib::object::ObjectRef;
    /// use silver_core::ObjectID;
    ///
    /// let id = ObjectID::new([0u8; 64]);
    /// let obj_ref = ObjectRef::new(id, 1);
    /// let next_ref = obj_ref.next_version();
    /// assert_eq!(next_ref.version(), 2);
    /// ```
    pub fn next_version(&self) -> Self {
        Self {
            id: self.id,
            version: self.version + 1,
        }
    }
}

impl fmt::Display for ObjectRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ObjectRef({}:{})", self.id, self.version)
    }
}

/// Object ownership type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Owner {
    /// Owned by a single address
    AddressOwner(SilverAddress),
    /// Shared object accessible by any transaction
    Shared {
        /// Initial version when object became shared
        initial_shared_version: u64,
    },
    /// Immutable object that cannot be modified
    Immutable,
    /// Wrapped in another object
    ObjectOwner(ObjectID),
}

impl Owner {
    /// Check if the owner is an address
    ///
    /// # Examples
    ///
    /// ```
    /// use quantum_stdlib::object::Owner;
    /// use silver_core::SilverAddress;
    ///
    /// let addr = SilverAddress::new([0u8; 64]);
    /// let owner = Owner::AddressOwner(addr);
    /// assert!(owner.is_address_owned());
    /// ```
    pub fn is_address_owned(&self) -> bool {
        matches!(self, Owner::AddressOwner(_))
    }

    /// Check if the object is shared
    ///
    /// # Examples
    ///
    /// ```
    /// use quantum_stdlib::object::Owner;
    ///
    /// let owner = Owner::Shared { initial_shared_version: 1 };
    /// assert!(owner.is_shared());
    /// ```
    pub fn is_shared(&self) -> bool {
        matches!(self, Owner::Shared { .. })
    }

    /// Check if the object is immutable
    ///
    /// # Examples
    ///
    /// ```
    /// use quantum_stdlib::object::Owner;
    ///
    /// let owner = Owner::Immutable;
    /// assert!(owner.is_immutable());
    /// ```
    pub fn is_immutable(&self) -> bool {
        matches!(self, Owner::Immutable)
    }

    /// Check if the object is owned by another object
    ///
    /// # Examples
    ///
    /// ```
    /// use quantum_stdlib::object::Owner;
    /// use silver_core::ObjectID;
    ///
    /// let parent_id = ObjectID::new([0u8; 64]);
    /// let owner = Owner::ObjectOwner(parent_id);
    /// assert!(owner.is_object_owned());
    /// ```
    pub fn is_object_owned(&self) -> bool {
        matches!(self, Owner::ObjectOwner(_))
    }

    /// Get the address owner if this is an address-owned object
    ///
    /// # Examples
    ///
    /// ```
    /// use quantum_stdlib::object::Owner;
    /// use silver_core::SilverAddress;
    ///
    /// let addr = SilverAddress::new([0u8; 64]);
    /// let owner = Owner::AddressOwner(addr);
    /// assert_eq!(owner.as_address(), Some(&addr));
    /// ```
    pub fn as_address(&self) -> std::option::Option<&SilverAddress> {
        match self {
            Owner::AddressOwner(addr) => std::option::Option::Some(addr),
            _ => std::option::Option::None,
        }
    }

    /// Get the parent object ID if this is an object-owned object
    ///
    /// # Examples
    ///
    /// ```
    /// use quantum_stdlib::object::Owner;
    /// use silver_core::ObjectID;
    ///
    /// let parent_id = ObjectID::new([0u8; 64]);
    /// let owner = Owner::ObjectOwner(parent_id);
    /// assert_eq!(owner.as_object(), Some(&parent_id));
    /// ```
    pub fn as_object(&self) -> std::option::Option<&ObjectID> {
        match self {
            Owner::ObjectOwner(id) => std::option::Option::Some(id),
            _ => std::option::Option::None,
        }
    }
}

impl fmt::Display for Owner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Owner::AddressOwner(addr) => write!(f, "AddressOwner({})", addr),
            Owner::Shared {
                initial_shared_version,
            } => write!(f, "Shared(v{})", initial_shared_version),
            Owner::Immutable => write!(f, "Immutable"),
            Owner::ObjectOwner(id) => write!(f, "ObjectOwner({})", id),
        }
    }
}

/// Object metadata
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ObjectMetadata {
    /// Object reference (ID + version)
    pub object_ref: ObjectRef,
    /// Object owner
    pub owner: Owner,
    /// Object type (module::struct)
    pub object_type: String,
}

impl ObjectMetadata {
    /// Create new object metadata
    ///
    /// # Arguments
    ///
    /// * `object_ref` - Object reference
    /// * `owner` - Object owner
    /// * `object_type` - Object type string
    ///
    /// # Examples
    ///
    /// ```
    /// use quantum_stdlib::object::{ObjectMetadata, ObjectRef, Owner};
    /// use silver_core::{ObjectID, SilverAddress};
    ///
    /// let id = ObjectID::new([0u8; 64]);
    /// let obj_ref = ObjectRef::new(id, 1);
    /// let addr = SilverAddress::new([0u8; 64]);
    /// let owner = Owner::AddressOwner(addr);
    /// let metadata = ObjectMetadata::new(obj_ref, owner, "coin::Coin".to_string());
    /// assert_eq!(metadata.object_type, "coin::Coin");
    /// ```
    pub fn new(object_ref: ObjectRef, owner: Owner, object_type: String) -> Self {
        Self {
            object_ref,
            owner,
            object_type,
        }
    }

    /// Get the object ID
    pub fn id(&self) -> &ObjectID {
        &self.object_ref.id
    }

    /// Get the object version
    pub fn version(&self) -> u64 {
        self.object_ref.version
    }

    /// Check if the object is owned by the given address
    ///
    /// # Arguments
    ///
    /// * `address` - Address to check
    ///
    /// # Examples
    ///
    /// ```
    /// use quantum_stdlib::object::{ObjectMetadata, ObjectRef, Owner};
    /// use silver_core::{ObjectID, SilverAddress};
    ///
    /// let id = ObjectID::new([0u8; 64]);
    /// let obj_ref = ObjectRef::new(id, 1);
    /// let addr = SilverAddress::new([0u8; 64]);
    /// let owner = Owner::AddressOwner(addr);
    /// let metadata = ObjectMetadata::new(obj_ref, owner, "coin::Coin".to_string());
    /// assert!(metadata.is_owned_by(&addr));
    /// ```
    pub fn is_owned_by(&self, address: &SilverAddress) -> bool {
        match &self.owner {
            Owner::AddressOwner(owner_addr) => owner_addr == address,
            _ => false,
        }
    }

    /// Check if the object can be modified
    ///
    /// Immutable objects cannot be modified
    pub fn is_mutable(&self) -> bool {
        !self.owner.is_immutable()
    }
}

impl fmt::Display for ObjectMetadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ObjectMetadata {{ ref: {}, owner: {}, type: {} }}",
            self.object_ref, self.owner, self.object_type
        )
    }
}

/// Utility functions for object operations
pub mod utils {
    use super::*;

    /// Generate a new object ID from transaction context
    ///
    /// This would typically be called by the VM runtime with proper context
    pub fn generate_object_id(tx_digest: &[u8; 64], index: u64) -> ObjectID {
        use blake3::Hasher;

        let mut hasher = Hasher::new();
        hasher.update(tx_digest);
        hasher.update(&index.to_le_bytes());

        let hash = hasher.finalize();
        let mut id_bytes = [0u8; 64];
        // Extend 32-byte Blake3 hash to 64 bytes by hashing again
        let extended = blake3::hash(hash.as_bytes());
        id_bytes[..32].copy_from_slice(hash.as_bytes());
        id_bytes[32..].copy_from_slice(extended.as_bytes());

        ObjectID::new(id_bytes)
    }

    /// Check if an object reference is valid
    ///
    /// # Arguments
    ///
    /// * `obj_ref` - Object reference to validate
    ///
    /// # Returns
    ///
    /// * `true` - If the reference is valid (version > 0)
    /// * `false` - If the reference is invalid
    pub fn is_valid_object_ref(obj_ref: &ObjectRef) -> bool {
        obj_ref.version > 0
    }

    /// Compare two object references for ordering
    ///
    /// Objects are ordered first by ID, then by version
    pub fn compare_object_refs(a: &ObjectRef, b: &ObjectRef) -> std::cmp::Ordering {
        match a.id.cmp(&b.id) {
            std::cmp::Ordering::Equal => a.version.cmp(&b.version),
            other => other,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_object_ref_creation() {
        let id = ObjectID::new([0u8; 64]);
        let obj_ref = ObjectRef::new(id, 1);
        assert_eq!(obj_ref.id(), &id);
        assert_eq!(obj_ref.version(), 1);
    }

    #[test]
    fn test_object_ref_increment() {
        let id = ObjectID::new([0u8; 64]);
        let mut obj_ref = ObjectRef::new(id, 1);
        obj_ref.increment_version();
        assert_eq!(obj_ref.version(), 2);

        let next = obj_ref.next_version();
        assert_eq!(next.version(), 3);
        assert_eq!(obj_ref.version(), 2); // Original unchanged
    }

    #[test]
    fn test_owner_types() {
        let addr = SilverAddress::new([0u8; 64]);
        let owner1 = Owner::AddressOwner(addr);
        assert!(owner1.is_address_owned());
        assert!(!owner1.is_shared());
        assert!(!owner1.is_immutable());

        let owner2 = Owner::Shared {
            initial_shared_version: 1,
        };
        assert!(!owner2.is_address_owned());
        assert!(owner2.is_shared());
        assert!(!owner2.is_immutable());

        let owner3 = Owner::Immutable;
        assert!(!owner3.is_address_owned());
        assert!(!owner3.is_shared());
        assert!(owner3.is_immutable());

        let parent_id = ObjectID::new([1u8; 64]);
        let owner4 = Owner::ObjectOwner(parent_id);
        assert!(owner4.is_object_owned());
        assert_eq!(owner4.as_object(), Some(&parent_id));
    }

    #[test]
    fn test_object_metadata() {
        let id = ObjectID::new([0u8; 64]);
        let obj_ref = ObjectRef::new(id, 1);
        let addr = SilverAddress::new([0u8; 64]);
        let owner = Owner::AddressOwner(addr);
        let metadata = ObjectMetadata::new(obj_ref, owner, "coin::Coin".to_string());

        assert_eq!(metadata.id(), &id);
        assert_eq!(metadata.version(), 1);
        assert_eq!(metadata.object_type, "coin::Coin");
        assert!(metadata.is_owned_by(&addr));
        assert!(metadata.is_mutable());
    }

    #[test]
    fn test_immutable_object() {
        let id = ObjectID::new([0u8; 64]);
        let obj_ref = ObjectRef::new(id, 1);
        let owner = Owner::Immutable;
        let metadata = ObjectMetadata::new(obj_ref, owner, "coin::Coin".to_string());

        assert!(!metadata.is_mutable());
    }

    #[test]
    fn test_generate_object_id() {
        let tx_digest = [1u8; 64];
        let id1 = utils::generate_object_id(&tx_digest, 0);
        let id2 = utils::generate_object_id(&tx_digest, 1);

        // Different indices should produce different IDs
        assert_ne!(id1, id2);

        // Same inputs should produce same ID
        let id3 = utils::generate_object_id(&tx_digest, 0);
        assert_eq!(id1, id3);
    }

    #[test]
    fn test_is_valid_object_ref() {
        let id = ObjectID::new([0u8; 64]);
        let valid_ref = ObjectRef::new(id, 1);
        assert!(utils::is_valid_object_ref(&valid_ref));

        let invalid_ref = ObjectRef::new(id, 0);
        assert!(!utils::is_valid_object_ref(&invalid_ref));
    }

    #[test]
    fn test_compare_object_refs() {
        let id1 = ObjectID::new([0u8; 64]);
        let id2 = ObjectID::new([1u8; 64]);

        let ref1 = ObjectRef::new(id1, 1);
        let ref2 = ObjectRef::new(id1, 2);
        let ref3 = ObjectRef::new(id2, 1);

        // Same ID, different version
        assert_eq!(
            utils::compare_object_refs(&ref1, &ref2),
            std::cmp::Ordering::Less
        );

        // Different ID
        assert_eq!(
            utils::compare_object_refs(&ref1, &ref3),
            std::cmp::Ordering::Less
        );

        // Same ref
        assert_eq!(
            utils::compare_object_refs(&ref1, &ref1),
            std::cmp::Ordering::Equal
        );
    }
}
