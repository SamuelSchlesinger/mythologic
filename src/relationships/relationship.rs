use serde::{Serialize, Deserialize};
use crate::core::{MythId, Metadata, MythEntity};

/// Represents a relationship between mythological entities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relationship {
    /// Unique identifier
    id: MythId,
    /// Name of the relationship
    name: String,
    /// Description of the relationship
    description: String,
    /// Source entity ID
    source_id: MythId,
    /// Target entity ID
    target_id: MythId,
    /// Type of relationship
    relationship_type: RelationshipType,
    /// Strength or importance of the relationship (0.0-1.0)
    strength: f32,
    /// Whether the relationship is bidirectional
    bidirectional: bool,
    /// Metadata
    metadata: Metadata,
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

impl MythEntity for Relationship {
    fn id(&self) -> &MythId {
        &self.id
    }
    
    fn name(&self) -> &str {
        &self.name
    }
    
    fn metadata(&self) -> &Metadata {
        &self.metadata
    }
    
    fn metadata_mut(&mut self) -> &mut Metadata {
        &mut self.metadata
    }
    
    fn entity_type(&self) -> &'static str {
        "Relationship"
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
