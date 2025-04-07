use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use crate::core::MythId;
use crate::relationships::{Relationship, RelationshipType, Relatable, Invertible, Properties, RelationshipBuilder};

/// Represents a transformation relationship between mythological entities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransformationRelationship {
    /// Base relationship
    pub relationship: Relationship,
    /// Type of transformation
    pub transformation_type: TransformationType,
    /// Cause of the transformation
    pub cause: String,
    /// Whether the transformation is permanent
    pub permanent: bool,
    /// Whether the transformation is reversible
    pub reversible: bool,
    /// Additional properties for extensibility
    #[serde(default)]
    pub properties: HashMap<String, String>,
}

impl TransformationRelationship {
    /// Create a new transformation relationship
    pub fn new(
        name: &str, 
        description: &str, 
        source_id: MythId, 
        target_id: MythId, 
        transformation_type: TransformationType,
        cause: &str
    ) -> Self {
        let relationship = RelationshipBuilder::new(
            name, 
            description, 
            source_id, 
            target_id, 
            RelationshipType::Transformation
        )
        .bidirectional(false)  // Transformations are typically one-way
        .build();
        
        Self {
            relationship,
            transformation_type,
            cause: cause.to_string(),
            permanent: true,
            reversible: false,
            properties: HashMap::new(),
        }
    }
    
    /// Get the transformation type
    pub fn transformation_type(&self) -> &TransformationType {
        &self.transformation_type
    }
    
    /// Set the transformation type
    pub fn set_transformation_type(&mut self, transformation_type: TransformationType) {
        self.transformation_type = transformation_type;
    }
    
    /// Get the cause
    pub fn cause(&self) -> &str {
        &self.cause
    }
    
    /// Set the cause
    pub fn set_cause(&mut self, cause: &str) {
        self.cause = cause.to_string();
    }
    
    /// Check if the transformation is permanent
    pub fn is_permanent(&self) -> bool {
        self.permanent
    }
    
    /// Set whether the transformation is permanent
    pub fn set_permanent(&mut self, permanent: bool) {
        self.permanent = permanent;
    }
    
    /// Check if the transformation is reversible
    pub fn is_reversible(&self) -> bool {
        self.reversible
    }
    
    /// Set whether the transformation is reversible
    pub fn set_reversible(&mut self, reversible: bool) {
        self.reversible = reversible;
    }
    
    /// Create a builder for configuring a new transformation relationship
    pub fn builder(
        name: &str, 
        description: &str, 
        source_id: MythId, 
        target_id: MythId, 
        transformation_type: TransformationType,
        cause: &str
    ) -> TransformationRelationshipBuilder {
        TransformationRelationshipBuilder {
            name: name.to_string(),
            description: description.to_string(),
            source_id,
            target_id,
            transformation_type,
            cause: cause.to_string(),
            permanent: true,
            reversible: false,
            bidirectional: false,
            strength: 0.5,
            properties: HashMap::new(),
        }
    }
}

/// Builder for transformation relationships
pub struct TransformationRelationshipBuilder {
    name: String,
    description: String,
    source_id: MythId,
    target_id: MythId,
    transformation_type: TransformationType,
    cause: String,
    permanent: bool,
    reversible: bool,
    bidirectional: bool,
    strength: f32,
    properties: HashMap<String, String>,
}

impl TransformationRelationshipBuilder {
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
    
    /// Set whether the transformation is permanent
    pub fn permanent(mut self, permanent: bool) -> Self {
        self.permanent = permanent;
        self
    }
    
    /// Set whether the transformation is reversible
    pub fn reversible(mut self, reversible: bool) -> Self {
        self.reversible = reversible;
        self
    }
    
    /// Add a custom property
    pub fn property(mut self, key: &str, value: &str) -> Self {
        self.properties.insert(key.to_string(), value.to_string());
        self
    }
    
    /// Build the final relationship
    pub fn build(self) -> TransformationRelationship {
        let relationship = RelationshipBuilder::new(
            &self.name,
            &self.description,
            self.source_id,
            self.target_id,
            RelationshipType::Transformation
        )
        .bidirectional(self.bidirectional)
        .strength(self.strength)
        .build();
        
        TransformationRelationship {
            relationship,
            transformation_type: self.transformation_type,
            cause: self.cause,
            permanent: self.permanent,
            reversible: self.reversible,
            properties: self.properties,
        }
    }
}

impl Relatable for TransformationRelationship {
    fn base(&self) -> &Relationship {
        &self.relationship
    }
    
    fn base_mut(&mut self) -> &mut Relationship {
        &mut self.relationship
    }
}

impl Properties for TransformationRelationship {
    fn get_property(&self, name: &str) -> Option<&str> {
        // First check specialized fields
        match name {
            "cause" => Some(&self.cause),
            "permanent" => Some(if self.permanent { "true" } else { "false" }),
            "reversible" => Some(if self.reversible { "true" } else { "false" }),
            // Then check the general properties map
            _ => self.properties.get(name).map(|s| s.as_str()),
        }
    }
    
    fn set_property(&mut self, name: &str, value: &str) {
        // Handle specialized fields
        match name {
            "cause" => self.cause = value.to_string(),
            "permanent" => self.permanent = value.parse::<bool>().unwrap_or(true),
            "reversible" => self.reversible = value.parse::<bool>().unwrap_or(false),
            // Otherwise store in the general properties map
            _ => { self.properties.insert(name.to_string(), value.to_string()); }
        }
    }
}

/// Type of transformation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransformationType {
    Shapeshifting,
    Petrification,
    Apotheosis,
    Curse,
    Blessing,
    Reincarnation,
    Metamorphosis,
    Other(String),
}

impl Invertible for TransformationType {
    /// Check if this type of relationship is typically bidirectional
    fn is_typically_bidirectional(&self) -> bool {
        false  // Transformations are typically one-way
    }
    
    /// Get the inverse relationship type
    fn inverse(&self) -> Self {
        match self {
            Self::Curse => Self::Blessing,
            Self::Blessing => Self::Curse,
            // Most transformation types don't have a clear inverse
            _ => self.clone(),
        }
    }
}
