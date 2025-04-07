use serde::{Serialize, Deserialize};
use std::collections::HashSet;
use crate::core::{MythId, Metadata};

/// Represents a cultural context for mythological entities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Culture {
    /// Unique identifier
    pub id: MythId,
    /// Name of the culture
    pub name: String,
    /// Description of the culture
    pub description: String,
    /// Geographic regions associated with this culture
    pub regions: HashSet<String>,
    /// Historical time periods
    pub time_periods: Vec<TimePeriod>,
    /// Major influences on this culture
    pub influences: Vec<String>,
    /// Languages associated with this culture
    pub languages: HashSet<String>,
    /// Relationships with other entities
    pub relationships: Vec<MythId>,
    /// Metadata
    pub metadata: Metadata,
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

// Trait implementations removed as we're using the enum approach

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
