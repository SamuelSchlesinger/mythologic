use serde::{Serialize, Deserialize};
use std::collections::HashSet;
use crate::core::{MythId, Metadata, CultureId, RegionId, LanguageId};

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
    pub regions: HashSet<RegionId>,
    /// Historical time periods
    pub time_periods: Vec<TimePeriod>,
    /// Major influences on this culture
    pub influences: Vec<CultureId>,
    /// Languages associated with this culture
    pub languages: HashSet<LanguageId>,
    /// Cultural practices (rituals, traditions, customs)
    pub cultural_practices: Vec<String>,
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
            cultural_practices: Vec::new(),
            relationships: Vec::new(),
            metadata: Metadata::new(),
        }
    }
    
    /// Add a region
    pub fn add_region(&mut self, region: &str) {
        self.regions.insert(RegionId::new(region));
    }
    
    /// Add a time period
    pub fn add_time_period(&mut self, time_period: TimePeriod) {
        self.time_periods.push(time_period);
    }
    
    /// Add an influence
    pub fn add_influence(&mut self, influence: &str) {
        self.influences.push(CultureId::new(influence));
    }
    
    /// Add a language
    pub fn add_language(&mut self, language: &str) {
        self.languages.insert(LanguageId::new(language));
    }
    
    /// Get the regions
    pub fn regions(&self) -> &HashSet<RegionId> {
        &self.regions
    }
    
    /// Get the time periods
    pub fn time_periods(&self) -> &[TimePeriod] {
        &self.time_periods
    }
    
    /// Get the influences
    pub fn influences(&self) -> &[CultureId] {
        &self.influences
    }
    
    /// Get the languages
    pub fn languages(&self) -> &HashSet<LanguageId> {
        &self.languages
    }
    
    /// Add a cultural practice
    pub fn add_cultural_practice(&mut self, practice: &str) {
        self.cultural_practices.push(practice.to_string());
    }
    
    /// Get the cultural practices
    pub fn cultural_practices(&self) -> &[String] {
        &self.cultural_practices
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
