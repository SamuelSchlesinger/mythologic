use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use crate::core::MythId;
use crate::relationships::{Relationship, RelationshipType, Relatable, Invertible, Properties, RelationshipBuilder};

/// Represents a family relationship between mythological entities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FamilyRelationship {
    /// Base relationship
    pub relationship: Relationship,
    /// Type of family relationship
    pub family_type: FamilyRelationshipType,
    /// Additional properties for extensibility
    #[serde(default)]
    pub properties: HashMap<String, String>,
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
        let relationship_builder = RelationshipBuilder::new(
            name, 
            description, 
            source_id, 
            target_id, 
            RelationshipType::Family
        );
        
        // Apply bidirectionality based on relationship type
        let relationship = relationship_builder
            .bidirectional(family_type.is_typically_bidirectional())
            .build();
            
        Self {
            relationship,
            family_type,
            properties: HashMap::new(),
        }
    }
    
    /// Get the family relationship type
    pub fn family_type(&self) -> &FamilyRelationshipType {
        &self.family_type
    }
    
    /// Set the family relationship type
    pub fn set_family_type(&mut self, family_type: FamilyRelationshipType) {
        let is_bidirectional = family_type.is_typically_bidirectional();
        self.family_type = family_type;
        // Update bidirectionality based on new relationship type
        self.set_bidirectional(is_bidirectional);
    }
    
    /// Create a builder for configuring a new family relationship
    pub fn builder(
        name: &str, 
        description: &str, 
        source_id: MythId, 
        target_id: MythId, 
        family_type: FamilyRelationshipType
    ) -> FamilyRelationshipBuilder {
        let is_bidirectional = family_type.is_typically_bidirectional();
        FamilyRelationshipBuilder {
            name: name.to_string(),
            description: description.to_string(),
            source_id,
            target_id,
            family_type,
            bidirectional: is_bidirectional,
            strength: 0.5,
            properties: HashMap::new(),
        }
    }
}

/// Builder for family relationships
pub struct FamilyRelationshipBuilder {
    name: String,
    description: String,
    source_id: MythId,
    target_id: MythId,
    family_type: FamilyRelationshipType,
    bidirectional: bool,
    strength: f32,
    properties: HashMap<String, String>,
}

impl FamilyRelationshipBuilder {
    /// Set the relationship strength
    pub fn strength(mut self, strength: f32) -> Self {
        self.strength = strength;
        self
    }
    
    /// Set whether the relationship is bidirectional
    pub fn bidirectional(mut self, bidirectional: bool) -> Self {
        self.bidirectional = bidirectional;
        self
    }
    
    /// Add a custom property
    pub fn property(mut self, key: &str, value: &str) -> Self {
        self.properties.insert(key.to_string(), value.to_string());
        self
    }
    
    /// Build the final relationship
    pub fn build(self) -> FamilyRelationship {
        let relationship_builder = RelationshipBuilder::new(
            &self.name,
            &self.description,
            self.source_id,
            self.target_id,
            RelationshipType::Family
        )
        .bidirectional(self.bidirectional)
        .strength(self.strength);
        
        FamilyRelationship {
            relationship: relationship_builder.build(),
            family_type: self.family_type,
            properties: self.properties,
        }
    }
}

impl Relatable for FamilyRelationship {
    fn base(&self) -> &Relationship {
        &self.relationship
    }
    
    fn base_mut(&mut self) -> &mut Relationship {
        &mut self.relationship
    }
}

impl Properties for FamilyRelationship {
    fn get_property(&self, name: &str) -> Option<&str> {
        self.properties.get(name).map(|s| s.as_str())
    }
    
    fn set_property(&mut self, name: &str, value: &str) {
        self.properties.insert(name.to_string(), value.to_string());
    }
}

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

impl Invertible for FamilyRelationshipType {
    /// Check if this type of relationship is typically bidirectional
    fn is_typically_bidirectional(&self) -> bool {
        match self {
            Self::Sibling => true,
            Self::Spouse => true,
            Self::Twin => true,
            Self::Cousin => true,
            _ => false,
        }
    }
    
    /// Get the inverse relationship type
    fn inverse(&self) -> Self {
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
