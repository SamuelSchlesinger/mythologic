use serde::{Serialize, Deserialize};
use crate::core::{MythId, Metadata};

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

// Trait implementations removed as we're using the enum approach

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
