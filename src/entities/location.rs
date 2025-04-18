use serde::{Serialize, Deserialize};
use crate::core::{MythId, Metadata, CultureId};

/// Represents a mythological location or realm
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    /// Unique identifier
    pub id: MythId,
    /// Primary name
    pub name: String,
    /// Description of the location
    pub description: String,
    /// Cultural origin
    pub culture: CultureId,
    /// Type of location
    pub location_type: LocationType,
    /// Physical or metaphysical characteristics
    pub characteristics: Vec<String>,
    /// Accessibility (how can it be reached)
    pub accessibility: Vec<String>,
    /// Relationships with other entities
    pub relationships: Vec<MythId>,
    /// Metadata
    pub metadata: Metadata,
}

impl Location {
    /// Create a new location
    pub fn new(name: &str, description: &str, culture: &str) -> Self {
        Self {
            id: MythId::new(),
            name: name.to_string(),
            description: description.to_string(),
            culture: CultureId::new(culture),
            location_type: LocationType::Unknown,
            characteristics: Vec::new(),
            accessibility: Vec::new(),
            relationships: Vec::new(),
            metadata: Metadata::new(),
        }
    }
    
    /// Add a characteristic
    pub fn add_characteristic(&mut self, characteristic: &str) {
        self.characteristics.push(characteristic.to_string());
    }
    
    /// Add an accessibility method
    pub fn add_accessibility(&mut self, access: &str) {
        self.accessibility.push(access.to_string());
    }
    
    /// Set the location type
    pub fn set_location_type(&mut self, location_type: LocationType) {
        self.location_type = location_type;
    }
    
    /// Get the culture
    pub fn culture(&self) -> &CultureId {
        &self.culture
    }
    
    /// Get the location type
    pub fn location_type(&self) -> &LocationType {
        &self.location_type
    }
    
    /// Get the characteristics
    pub fn characteristics(&self) -> &[String] {
        &self.characteristics
    }
    
    /// Get the accessibility methods
    pub fn accessibility(&self) -> &[String] {
        &self.accessibility
    }
}

// Trait implementations removed as we're using the enum approach

/// Type of mythological location
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LocationType {
    Afterlife,
    Heaven,
    Underworld,
    Mountain,
    Forest,
    Sea,
    Island,
    City,
    Temple,
    Cosmic,
    Liminal,
    Unknown,
    Other(String),
}
