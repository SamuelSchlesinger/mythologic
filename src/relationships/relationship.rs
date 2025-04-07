use serde::{Serialize, Deserialize};
use std::fmt::Debug;
use crate::core::{MythId, Metadata};

/// Common trait for all relationship types
pub trait Relatable: Debug + Clone {
    /// Get the base relationship information
    fn base(&self) -> &Relationship;
    
    /// Get mutable access to the base relationship information
    fn base_mut(&mut self) -> &mut Relationship;
    
    /// Get the relationship ID
    fn id(&self) -> &MythId {
        &self.base().id
    }
    
    /// Get the relationship name
    fn name(&self) -> &str {
        &self.base().name
    }
    
    /// Get the relationship description
    fn description(&self) -> &str {
        &self.base().description
    }
    
    /// Get the source entity ID
    fn source_id(&self) -> &MythId {
        &self.base().source_id
    }
    
    /// Get the target entity ID
    fn target_id(&self) -> &MythId {
        &self.base().target_id
    }
    
    /// Get the relationship type
    fn relationship_type(&self) -> &RelationshipType {
        &self.base().relationship_type
    }
    
    /// Get the relationship strength
    fn strength(&self) -> f32 {
        self.base().strength
    }
    
    /// Set the relationship strength
    fn set_strength(&mut self, strength: f32) {
        self.base_mut().set_strength(strength);
    }
    
    /// Check if the relationship is bidirectional
    fn is_bidirectional(&self) -> bool {
        self.base().bidirectional
    }
    
    /// Set whether the relationship is bidirectional
    fn set_bidirectional(&mut self, bidirectional: bool) {
        self.base_mut().set_bidirectional(bidirectional);
    }
    
    /// Get the relationship metadata
    fn metadata(&self) -> &Metadata {
        &self.base().metadata
    }
    
    /// Get mutable access to the relationship metadata
    fn metadata_mut(&mut self) -> &mut Metadata {
        &mut self.base_mut().metadata
    }
}

/// Shared trait for relationship types that have an inverse
pub trait Invertible {
    /// Get the inverse relationship type
    fn inverse(&self) -> Self;
    
    /// Check if this relationship type is typically bidirectional
    fn is_typically_bidirectional(&self) -> bool;
}

/// Shared trait for relationships that can contain additional properties
pub trait Properties {
    /// Get a property value by name
    fn get_property(&self, name: &str) -> Option<&str>;
    
    /// Set a property value
    fn set_property(&mut self, name: &str, value: &str);
}

/// Represents a relationship between mythological entities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relationship {
    /// Unique identifier
    pub id: MythId,
    /// Name of the relationship
    pub name: String,
    /// Description of the relationship
    pub description: String,
    /// Source entity ID
    pub source_id: MythId,
    /// Target entity ID
    pub target_id: MythId,
    /// Type of relationship
    pub relationship_type: RelationshipType,
    /// Strength or importance of the relationship (0.0-1.0)
    pub strength: f32,
    /// Whether the relationship is bidirectional
    pub bidirectional: bool,
    /// Metadata
    pub metadata: Metadata,
}

impl Relationship {
    /// Create a new relationship
    pub fn new(
        name: &str, 
        description: &str, 
        source_id: MythId, 
        target_id: MythId, 
        relationship_type: RelationshipType
    ) -> Self {
        Self {
            id: MythId::new(),
            name: name.to_string(),
            description: description.to_string(),
            source_id,
            target_id,
            relationship_type,
            strength: 0.5,
            bidirectional: false,
            metadata: Metadata::new(),
        }
    }
    
    /// Get the source entity ID
    pub fn source_id(&self) -> &MythId {
        &self.source_id
    }
    
    /// Get the target entity ID
    pub fn target_id(&self) -> &MythId {
        &self.target_id
    }
    
    /// Get the relationship type
    pub fn relationship_type(&self) -> &RelationshipType {
        &self.relationship_type
    }
    
    /// Set the relationship strength
    pub fn set_strength(&mut self, strength: f32) {
        self.strength = strength.max(0.0).min(1.0);
    }
    
    /// Get the relationship strength
    pub fn strength(&self) -> f32 {
        self.strength
    }
    
    /// Set whether the relationship is bidirectional
    pub fn set_bidirectional(&mut self, bidirectional: bool) {
        self.bidirectional = bidirectional;
    }
    
    /// Check if the relationship is bidirectional
    pub fn is_bidirectional(&self) -> bool {
        self.bidirectional
    }
}

/// Builder for creating relationships
pub struct RelationshipBuilder {
    relationship: Relationship,
}

impl RelationshipBuilder {
    /// Start building a new relationship
    pub fn new(
        name: &str, 
        description: &str, 
        source_id: MythId, 
        target_id: MythId, 
        relationship_type: RelationshipType
    ) -> Self {
        Self {
            relationship: Relationship::new(name, description, source_id, target_id, relationship_type),
        }
    }
    
    /// Set the relationship strength
    pub fn strength(mut self, strength: f32) -> Self {
        self.relationship.set_strength(strength);
        self
    }
    
    /// Set whether the relationship is bidirectional
    pub fn bidirectional(mut self, bidirectional: bool) -> Self {
        self.relationship.set_bidirectional(bidirectional);
        self
    }
    
    /// Complete the relationship building process
    pub fn build(self) -> Relationship {
        self.relationship
    }
}

/// Type of relationship between entities
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RelationshipType {
    Family,
    Alliance,
    Conflict,
    Creation,
    Transformation,
    Worship,
    Possession,
    Guardianship,
    Teaching,
    Unknown,
    Other(String),
}
