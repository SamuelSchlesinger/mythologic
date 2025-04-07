use serde::{Serialize, Deserialize};
use crate::core::MythId;
use crate::relationships::Relationship;

/// Represents a family relationship between mythological entities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FamilyRelationship {
    /// Base relationship
    pub relationship: Relationship,
    /// Type of family relationship
    pub family_type: FamilyRelationshipType,
}

impl FamilyRelationship {
    /// Create a new family relationship
    pub fn new(
        name: &str, 
        description: &str, 
        source_id: MythId, 
        target_id: MythId, 
        family_type: FamilyRelationshipType
    ) -> Self {
        let mut relationship = Relationship::new(
            name, 
            description, 
            source_id, 
            target_id, 
            crate::relationships::RelationshipType::Family
        );
        
        // Most family relationships are bidirectional
        if family_type.is_typically_bidirectional() {
            relationship.set_bidirectional(true);
        }
        
        Self {
            relationship,
            family_type,
        }
    }
    
    /// Get the family relationship type
    pub fn family_type(&self) -> &FamilyRelationshipType {
        &self.family_type
    }
    
    /// Set the family relationship type
    pub fn set_family_type(&mut self, family_type: FamilyRelationshipType) {
        self.family_type = family_type;
    }
}

// Trait implementations removed as we're using the enum approach

/// Type of family relationship
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FamilyRelationshipType {
    Parent,
    Child,
    Sibling,
    Spouse,
    Ancestor,
    Descendant,
    Twin,
    Cousin,
    Other(String),
}

impl FamilyRelationshipType {
    /// Check if this type of relationship is typically bidirectional
    pub fn is_typically_bidirectional(&self) -> bool {
        match self {
            Self::Sibling => true,
            Self::Spouse => true,
            Self::Twin => true,
            Self::Cousin => true,
            _ => false,
        }
    }
    
    /// Get the inverse relationship type
    pub fn inverse(&self) -> Self {
        match self {
            Self::Parent => Self::Child,
            Self::Child => Self::Parent,
            Self::Ancestor => Self::Descendant,
            Self::Descendant => Self::Ancestor,
            // These are their own inverses
            Self::Sibling => Self::Sibling,
            Self::Spouse => Self::Spouse,
            Self::Twin => Self::Twin,
            Self::Cousin => Self::Cousin,
            Self::Other(s) => Self::Other(s.clone()),
        }
    }
}
