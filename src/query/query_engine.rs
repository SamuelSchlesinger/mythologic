use std::collections::HashSet;
use crate::core::{MythId, MythEntity, MythOntology};
use crate::query::{QueryFilter, QueryResult, QueryResultSet};

/// Engine for querying the mythological ontology
pub struct QueryEngine<'a> {
    ontology: &'a MythOntology,
}

impl<'a> QueryEngine<'a> {
    /// Create a new query engine
    pub fn new(ontology: &'a MythOntology) -> Self {
        Self { ontology }
    }
    
    /// Query entities that match the given filters
    pub fn query(&self, filters: &[QueryFilter]) -> QueryResultSet {
        let mut result_ids = HashSet::new();
        let mut results = Vec::new();
        
        // Start with all entities
        for entity in self.ontology.all_entities() {
            let mut matches = true;
            
            // Check if entity matches all filters
            for filter in filters {
                if !filter.matches(entity) {
                    matches = false;
                    break;
                }
            }
            
            if matches {
                let id = entity.id().clone();
                if !result_ids.contains(&id) {
                    result_ids.insert(id.clone());
                    results.push(QueryResult {
                        id: id,
                        name: entity.name().to_string(),
                        entity_type: entity.entity_type().to_string(),
                    });
                }
            }
        }
        
        QueryResultSet { results }
    }
    
    /// Find entities related to the given entity
    pub fn find_related(&self, entity_id: &MythId) -> QueryResultSet {
        let mut result_ids = HashSet::new();
        let mut results = Vec::new();
        
        if let Some(entity) = self.ontology.get_entity(entity_id) {
            // First, check if the entity implements Relatable
            if let Some(relatable) = entity.downcast_ref::<dyn crate::core::Relatable>() {
                for related_id in relatable.relationships() {
                    if let Some(related_entity) = self.ontology.get_entity(&related_id) {
                        if !result_ids.contains(&related_id) {
                            result_ids.insert(related_id.clone());
                            results.push(QueryResult {
                                id: related_id,
                                name: related_entity.name().to_string(),
                                entity_type: related_entity.entity_type().to_string(),
                            });
                        }
                    }
                }
            }
        }
        
        QueryResultSet { results }
    }
    
    /// Find entities by name (partial match)
    pub fn find_by_name(&self, name: &str) -> QueryResultSet {
        let name_lower = name.to_lowercase();
        let mut result_ids = HashSet::new();
        let mut results = Vec::new();
        
        for entity in self.ontology.all_entities() {
            if entity.name().to_lowercase().contains(&name_lower) {
                let id = entity.id().clone();
                if !result_ids.contains(&id) {
                    result_ids.insert(id.clone());
                    results.push(QueryResult {
                        id: id,
                        name: entity.name().to_string(),
                        entity_type: entity.entity_type().to_string(),
                    });
                }
            }
        }
        
        QueryResultSet { results }
    }
    
    /// Find entities by type
    pub fn find_by_type(&self, entity_type: &str) -> QueryResultSet {
        let mut result_ids = HashSet::new();
        let mut results = Vec::new();
        
        for entity in self.ontology.all_entities() {
            if entity.entity_type() == entity_type {
                let id = entity.id().clone();
                if !result_ids.contains(&id) {
                    result_ids.insert(id.clone());
                    results.push(QueryResult {
                        id: id,
                        name: entity.name().to_string(),
                        entity_type: entity.entity_type().to_string(),
                    });
                }
            }
        }
        
        QueryResultSet { results }
    }
}
