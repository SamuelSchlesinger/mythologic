use serde::{Serialize, Deserialize};
use crate::core::{MythId, Metadata, MythEntity, Relatable};

/// Represents an abstract mythological concept
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Concept {
    /// Unique identifier
    id: MythId,
    /// Primary name
    name: String,
    /// Description of the concept
    description: String,
    /// Cultural origin
    culture: String,
    /// Type of concept
    concept_type: ConceptType,
    /// Manifestations or representations
    manifestations: Vec<String>,
    /// Relationships with other entities
    relationships: Vec<MythId>,
    /// Metadata
    metadata: Metadata,
}

impl Concept {
    /// Create a new concept
    pub fn new(name: &str, description: &str, culture: &str) -> Self {
        Self {
            id: MythId::new(),
            name: name.to_string(),
            description: description.to_string(),
            culture: culture.to_string(),
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
    pub fn culture(&self) -> &str {
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

impl MythEntity for Concept {
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
        "Concept"
    }
}

impl Relatable for Concept {
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
