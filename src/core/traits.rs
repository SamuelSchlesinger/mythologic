use crate::core::{MythId, Metadata};

/// A trait for any entity that can be part of the mythological ontology
pub trait MythEntity {
    /// Get the unique identifier for this entity
    fn id(&self) -> &MythId;
    
    /// Get the name of this entity
    fn name(&self) -> &str;
    
    /// Get the metadata for this entity
    fn metadata(&self) -> &Metadata;
    
    /// Get mutable metadata for this entity
    fn metadata_mut(&mut self) -> &mut Metadata;
    
    /// Get the entity type as a string
    fn entity_type(&self) -> &'static str;
    
    /// Get the relationships this entity has (default empty implementation)
    fn relationships(&self) -> Vec<MythId> {
        Vec::new()
    }
}

/// A trait for entities that can be related to other entities
pub trait Relatable {
    /// Get the relationships this entity has
    fn relationships(&self) -> Vec<MythId>;
    
    /// Add a relationship to this entity
    fn add_relationship(&mut self, relationship_id: MythId);
    
    /// Remove a relationship from this entity
    fn remove_relationship(&mut self, relationship_id: &MythId) -> bool;
}
