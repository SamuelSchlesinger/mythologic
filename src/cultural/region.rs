use serde::{Serialize, Deserialize};
use std::collections::HashSet;
use crate::core::{MythId, Metadata, CultureId, FeatureId, LocationId};

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
    pub cultures: HashSet<CultureId>,
    /// Geographic features
    pub features: Vec<FeatureId>,
    /// Modern-day equivalent locations
    pub modern_locations: HashSet<LocationId>,
    /// Significance in mythology
    pub significance: String,
    /// Relationships with other entities
    pub relationships: Vec<MythId>,
    /// Metadata
    pub metadata: Metadata,
}

impl MythologicalRegion {
    /// Create a new mythological region
    pub fn new(name: &str, description: &str, significance: Option<&str>) -> Self {
        Self {
            id: MythId::new(),
            name: name.to_string(),
            description: description.to_string(),
            cultures: HashSet::new(),
            features: Vec::new(),
            modern_locations: HashSet::new(),
            significance: significance.unwrap_or("Unknown significance").to_string(),
            relationships: Vec::new(),
            metadata: Metadata::new(),
        }
    }
    
    /// Alternative constructor with all parameters
    pub fn new_with_significance(name: &str, description: &str, significance: &str) -> Self {
        Self::new(name, description, Some(significance))
    }
    
    /// Add a culture
    pub fn add_culture(&mut self, culture: &str) {
        self.cultures.insert(CultureId::new(culture));
    }
    
    /// Add a geographic feature
    pub fn add_feature(&mut self, feature: &str) {
        self.features.push(FeatureId::new(feature));
    }
    
    /// Add a modern location
    pub fn add_modern_location(&mut self, location: &str) {
        self.modern_locations.insert(LocationId::new(location));
    }
    
    /// Get the cultures
    pub fn cultures(&self) -> &HashSet<CultureId> {
        &self.cultures
    }
    
    /// Get the features
    pub fn features(&self) -> &[FeatureId] {
        &self.features
    }
    
    /// Get the modern locations
    pub fn modern_locations(&self) -> &HashSet<LocationId> {
        &self.modern_locations
    }
    
    /// Get the significance
    pub fn significance(&self) -> &str {
        &self.significance
    }
}

// Trait implementations removed as we're using the enum approach
