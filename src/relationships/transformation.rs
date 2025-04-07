use serde::{Serialize, Deserialize};
use crate::relationships::Relationship;

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
        let relationship = Relationship::new(
            name, 
            description, 
            source_id, 
            target_id, 
            crate::relationships::RelationshipType::Transformation
        );
        
        Self {
            relationship,
            transformation_type,
            cause: cause.to_string(),
            permanent: true,
            reversible: false,
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
}

impl MythEntity for TransformationRelationship {
    fn id(&self) -> &MythId {
        self.relationship.id()
    }
    
    fn name(&self) -> &str {
        self.relationship.name()
    }
    
    fn metadata(&self) -> &Metadata {
        self.relationship.metadata()
    }
    
    fn metadata_mut(&mut self) -> &mut Metadata {
        self.relationship.metadata_mut()
    }
    
    fn entity_type(&self) -> &'static str {
        "TransformationRelationship"
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
