use std::collections::HashMap;
use crate::core::{MythId, MythEntity};

/// The main ontology that holds all mythological entities and their relationships
pub struct MythOntology {
    entities: HashMap<MythId, MythEntity>,
}

impl MythOntology {
    /// Create a new empty ontology
    pub fn new() -> Self {
        Self {
            entities: HashMap::new(),
        }
    }
    
    /// Add an entity to the ontology
    pub fn add_entity(&mut self, entity: MythEntity) {
        let id = entity.id().clone();
        self.entities.insert(id, entity);
    }
    
    /// Get an entity by its ID
    pub fn get_entity(&self, id: &MythId) -> Option<&MythEntity> {
        self.entities.get(id)
    }
    
    /// Get a mutable reference to an entity by its ID
    pub fn get_entity_mut(&mut self, id: &MythId) -> Option<&mut MythEntity> {
        self.entities.get_mut(id)
    }
    
    /// Remove an entity from the ontology
    pub fn remove_entity(&mut self, id: &MythId) -> Option<MythEntity> {
        self.entities.remove(id)
    }
    
    /// Get all entities in the ontology
    pub fn all_entities(&self) -> Vec<&MythEntity> {
        self.entities.values().collect()
    }
    
    /// Count the number of entities in the ontology
    pub fn entity_count(&self) -> usize {
        self.entities.len()
    }
}

impl Default for MythOntology {
    fn default() -> Self {
        Self::new()
    }
}
