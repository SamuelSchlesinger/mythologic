use serde::{Serialize, Deserialize};
use std::collections::HashSet;
use crate::core::{MythId, Metadata, MythEntity, Relatable};

/// Represents a cultural context for mythological entities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Culture {
    /// Unique identifier
    id: MythId,
    /// Name of the culture
    name: String,
    /// Description of the culture
    description: String,
    /// Geographic regions associated with this culture
    regions: HashSet<String>,
    /// Historical time periods
    time_periods: Vec<TimePeriod>,
    /// Major influences on this culture
    influences: Vec<String>,
    /// Languages associated with this culture
    languages: HashSet<String>,
    /// Relationships with other entities
    relationships: Vec<MythId>,
    /// Metadata
    metadata: Metadata,
}

impl Culture {
    /// Create a new culture
    pub fn new(name: &str, description: &str) -> Self {
        Self {
            id: MythId::new(),
            name: name.to_string(),
            description: description.to_string(),
            regions: HashSet::new(),
            time_periods: Vec::new(),
            influences: Vec::new(),
            languages: HashSet::new(),
            relationships: Vec::new(),
            metadata: Metadata::new(),
        }
    }
    
    /// Add a region
    pub fn add_region(&mut self, region: &str) {
        self.regions.insert(region.to_string());
    }
    
    /// Add a time period
    pub fn add_time_period(&mut self, time_period: TimePeriod) {
        self.time_periods.push(time_period);
    }
    
    /// Add an influence
    pub fn add_influence(&mut self, influence: &str) {
        self.influences.push(influence.to_string());
    }
    
    /// Add a language
    pub fn add_language(&mut self, language: &str) {
        self.languages.insert(language.to_string());
    }
    
    /// Get the regions
    pub fn regions(&self) -> &HashSet<String> {
        &self.regions
    }
    
    /// Get the time periods
    pub fn time_periods(&self) -> &[TimePeriod] {
        &self.time_periods
    }
    
    /// Get the influences
    pub fn influences(&self) -> &[String] {
        &self.influences
    }
    
    /// Get the languages
    pub fn languages(&self) -> &HashSet<String> {
        &self.languages
    }
}

impl MythEntity for Culture {
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
        "Culture"
    }
}

impl Relatable for Culture {
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

/// A historical time period
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TimePeriod {
    /// Name of the time period
    pub name: String,
    /// Start year (approximate, can be negative for BCE)
    pub start_year: Option<i32>,
    /// End year (approximate, can be negative for BCE)
    pub end_year: Option<i32>,
    /// Description of the time period
    pub description: Option<String>,
}
