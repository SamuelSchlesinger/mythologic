use serde::{Serialize, Deserialize};
use std::collections::HashSet;
use crate::core::{MythId, Metadata};

/// Represents a geographic region in mythology
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MythologicalRegion {
    /// Unique identifier
    pub id: MythId,
    /// Name of the region
    pub name: String,
    /// Description of the region
    pub description: String,
    /// Cultural associations
    pub cultures: HashSet<String>,
    /// Geographic features
    pub features: Vec<String>,
    /// Modern-day equivalent locations
    pub modern_locations: HashSet<String>,
    /// Significance in mythology
    pub significance: String,
    /// Relationships with other entities
    pub relationships: Vec<MythId>,
    /// Metadata
    pub metadata: Metadata,
}

impl MythologicalRegion {
    /// Create a new mythological region
    pub fn new(name: &str, description: &str, significance: &str) -> Self {
        Self {
            id: MythId::new(),
            name: name.to_string(),
            description: description.to_string(),
            cultures: HashSet::new(),
            features: Vec::new(),
            modern_locations: HashSet::new(),
            significance: significance.to_string(),
            relationships: Vec::new(),
            metadata: Metadata::new(),
        }
    }
    
    /// Add a culture
    pub fn add_culture(&mut self, culture: &str) {
        self.cultures.insert(culture.to_string());
    }
    
    /// Add a geographic feature
    pub fn add_feature(&mut self, feature: &str) {
        self.features.push(feature.to_string());
    }
    
    /// Add a modern location
    pub fn add_modern_location(&mut self, location: &str) {
        self.modern_locations.insert(location.to_string());
    }
    
    /// Get the cultures
    pub fn cultures(&self) -> &HashSet<String> {
        &self.cultures
    }
    
    /// Get the features
    pub fn features(&self) -> &[String] {
        &self.features
    }
    
    /// Get the modern locations
    pub fn modern_locations(&self) -> &HashSet<String> {
        &self.modern_locations
    }
    
    /// Get the significance
    pub fn significance(&self) -> &str {
        &self.significance
    }
}

impl MythEntity for MythologicalRegion {
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
        "MythologicalRegion"
    }
}

impl Relatable for MythologicalRegion {
    fn relationships(&self) -> Vec<MythId> {
        self.relationships.clone()
    }
    
    fn add_relationship(&mut self, relationship_id: MythId) {
        self.relationships.push(relationship_id);
    }
    
    fn remove_relationship(&mut self, relationship_id: &MythId) -> bool {
        let len = self.relationships.len();
        self.relationships.retain(|id| id != relationship_id);
        self.relationships.len() != len
    }
}
