use serde::{Serialize, Deserialize};
use crate::core::MythId;

/// A single query result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryResult {
    /// Entity ID
    pub id: MythId,
    /// Entity name
    pub name: String,
    /// Entity type
    pub entity_type: String,
}

/// A set of query results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryResultSet {
    /// The results
    pub results: Vec<QueryResult>,
}

impl QueryResultSet {
    /// Create an empty result set
    pub fn empty() -> Self {
        Self { results: Vec::new() }
    }
    
    /// Get the number of results
    pub fn count(&self) -> usize {
        self.results.len()
    }
    
    /// Check if the result set is empty
    pub fn is_empty(&self) -> bool {
        self.results.is_empty()
    }
    
    /// Get the first result, if any
    pub fn first(&self) -> Option<&QueryResult> {
        self.results.first()
    }
    
    /// Filter results by entity type
    pub fn filter_by_type(&self, entity_type: &str) -> Self {
        let results = self.results.iter()
            .filter(|r| r.entity_type == entity_type)
            .cloned()
            .collect();
        
        Self { results }
    }
    
    /// Sort results by name
    pub fn sort_by_name(&mut self) {
        self.results.sort_by(|a, b| a.name.cmp(&b.name));
    }
    
    /// Get entity IDs from the results
    pub fn entity_ids(&self) -> Vec<MythId> {
        self.results.iter().map(|r| r.id.clone()).collect()
    }
}
