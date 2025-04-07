use serde::{Serialize, Deserialize};
use crate::core::{MythId, Metadata, CultureId};

/// Represents an abstract mythological concept
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Concept {
    /// Unique identifier
    pub id: MythId,
    /// Primary name
    pub name: String,
    /// Description of the concept
    pub description: String,
    /// Cultural origin
    pub culture: CultureId,
    /// Type of concept
    pub concept_type: ConceptType,
    /// Manifestations or representations
    pub manifestations: Vec<String>,
    /// Relationships with other entities
    pub relationships: Vec<MythId>,
    /// Metadata
    pub metadata: Metadata,
}

impl Concept {
    /// Create a new concept
    pub fn new(name: &str, description: &str, culture: &str) -> Self {
        Self {
            id: MythId::new(),
            name: name.to_string(),
            description: description.to_string(),
            culture: CultureId::new(culture),
            concept_type: ConceptType::Unknown,
            manifestations: Vec::new(),
            relationships: Vec::new(),
            metadata: Metadata::new(),
        }
    }
    
    /// Add a manifestation
    pub fn add_manifestation(&mut self, manifestation: &str) {
        self.manifestations.push(manifestation.to_string());
    }
    
    /// Set the concept type
    pub fn set_concept_type(&mut self, concept_type: ConceptType) {
        self.concept_type = concept_type;
    }
    
    /// Get the culture
    pub fn culture(&self) -> &CultureId {
        &self.culture
    }
    
    /// Get the concept type
    pub fn concept_type(&self) -> &ConceptType {
        &self.concept_type
    }
    
    /// Get the manifestations
    pub fn manifestations(&self) -> &[String] {
        &self.manifestations
    }
}

// Trait implementations removed as we're using the enum approach

/// Type of mythological concept
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConceptType {
    Cosmology,
    Creation,
    Afterlife,
    Virtue,
    Vice,
    Fate,
    Time,
    Justice,
    Love,
    War,
    Unknown,
    Other(String),
}
