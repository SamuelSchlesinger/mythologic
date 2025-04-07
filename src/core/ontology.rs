use std::collections::HashMap;
use crate::core::{MythId, MythEntity};

/// The main ontology that holds all mythological entities and their relationships
pub struct MythOntology {
    entities: HashMap<MythId, Box<dyn MythEntity>>,
}

impl MythOntology {
    /// Create a new empty ontology
    pub fn new() -> Self {
        Self {
            entities: HashMap::new(),
        }
    }
    
    /// Add an entity to the ontology
    pub fn add_entity<T: MythEntity + 'static>(&mut self, entity: T) {
        let id = entity.id().clone();
        self.entities.insert(id, Box::new(entity));
    }
    
    /// Get an entity by its ID
    pub fn get_entity(&self, id: &MythId) -> Option<&dyn MythEntity> {
        self.entities.get(id).map(|e| e.as_ref())
    }
    
    /// Get a mutable reference to an entity by its ID
    pub fn get_entity_mut(&mut self, id: &MythId) -> Option<&mut dyn MythEntity> {
        self.entities.get_mut(id).map(|e| e.as_mut())
    }
    
    /// Remove an entity from the ontology
    pub fn remove_entity(&mut self, id: &MythId) -> Option<Box<dyn MythEntity>> {
        self.entities.remove(id)
    }
    
    /// Get all entities in the ontology
    pub fn all_entities(&self) -> Vec<&dyn MythEntity> {
        self.entities.values().map(|e| e.as_ref()).collect()
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
