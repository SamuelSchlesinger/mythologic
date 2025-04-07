//! # Universal Unique Identifiers
//!
//! This module provides the core `MythId` type which serves as a unique identifier
//! for all entities in the mythological ontology. Unlike the name-based IDs in
//! the `names` module which are used for referencing by name, `MythId` is a
//! UUID-based identifier that ensures global uniqueness.

use uuid::Uuid;
use std::fmt;
use serde::{Serialize, Deserialize};

/// A universally unique identifier for any entity in the mythological ontology.
///
/// `MythId` is based on UUID v4 (random) and is used to uniquely identify 
/// any entity, relationship, or cultural context within the system. Every
/// entity in the system has precisely one `MythId`, and these IDs are globally
/// unique across all entity types.
///
/// This type provides the foundation for:
/// - Uniquely identifying entities regardless of names or attributes
/// - Establishing relationships between entities
/// - Persisting and retrieving entities across systems
/// - Denoting entity identity separate from content
///
/// # Examples
///
/// ```
/// use mythologic::core::MythId;
///
/// // Generate a new random ID
/// let id1 = MythId::new();
/// let id2 = MythId::new();
///
/// // IDs are unique
/// assert_ne!(id1, id2);
///
/// // Convert to string and back
/// let id_str = id1.to_string();
/// let parsed_id = MythId::from_str(&id_str).unwrap();
/// assert_eq!(id1, parsed_id);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MythId(Uuid);

impl MythId {
    /// Create a new random MythId based on UUIDv4.
    ///
    /// This generates a cryptographically secure random ID that is
    /// extremely unlikely to collide with any other ID, even across
    /// different systems and databases.
    ///
    /// # Returns
    ///
    /// A new randomly generated `MythId`.
    ///
    /// # Examples
    ///
    /// ```
    /// use mythologic::core::MythId;
    ///
    /// let id = MythId::new();
    /// ```
    pub fn new() -> Self {
        MythId(Uuid::new_v4())
    }
    
    /// Create a MythId from a string representation.
    ///
    /// This parses a standard UUID string format (e.g., 
    /// "123e4567-e89b-12d3-a456-426614174000") into a `MythId`.
    ///
    /// # Arguments
    ///
    /// * `s` - A string slice representing a UUID
    ///
    /// # Returns
    ///
    /// A `Result` containing either the parsed `MythId` or a UUID parsing error.
    ///
    /// # Examples
    ///
    /// ```
    /// use mythologic::core::MythId;
    ///
    /// let id = MythId::from_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
    /// ```
    pub fn from_str(s: &str) -> Result<Self, uuid::Error> {
        Ok(MythId(Uuid::parse_str(s)?))
    }
    
    /// Get the string representation of this MythId.
    ///
    /// # Returns
    ///
    /// A standard hyphenated UUID string.
    ///
    /// # Examples
    ///
    /// ```
    /// use mythologic::core::MythId;
    ///
    /// let id = MythId::new();
    /// let id_str = id.to_string();
    /// // id_str will be something like "550e8400-e29b-41d4-a716-446655440000"
    /// ```
    pub fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl fmt::Display for MythId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Default for MythId {
    fn default() -> Self {
        Self::new()
    }
}
