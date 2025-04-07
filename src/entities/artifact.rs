use serde::{Serialize, Deserialize};
use crate::core::{MythId, Metadata, CultureId};

/// Represents a mythological artifact or object of power
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Artifact {
    /// Unique identifier
    pub id: MythId,
    /// Primary name
    pub name: String,
    /// Description of the artifact
    pub description: String,
    /// Cultural origin
    pub culture: CultureId,
    /// Type of artifact
    pub artifact_type: ArtifactType,
    /// Powers or properties
    pub powers: Vec<String>,
    /// Creator of the artifact
    pub creator: Option<String>,
    /// Current or last known owner
    pub owner: Option<String>,
    /// Relationships with other entities
    pub relationships: Vec<MythId>,
    /// Metadata
    pub metadata: Metadata,
}

impl Artifact {
    /// Create a new artifact
    pub fn new(name: &str, description: &str, culture: &str) -> Self {
        Self {
            id: MythId::new(),
            name: name.to_string(),
            description: description.to_string(),
            culture: CultureId::new(culture),
            artifact_type: ArtifactType::Unknown,
            powers: Vec::new(),
            creator: None,
            owner: None,
            relationships: Vec::new(),
            metadata: Metadata::new(),
        }
    }
    
    /// Add a power
    pub fn add_power(&mut self, power: &str) {
        self.powers.push(power.to_string());
    }
    
    /// Set the artifact type
    pub fn set_artifact_type(&mut self, artifact_type: ArtifactType) {
        self.artifact_type = artifact_type;
    }
    
    /// Set the creator
    pub fn set_creator(&mut self, creator: &str) {
        self.creator = Some(creator.to_string());
    }
    
    /// Set the owner
    pub fn set_owner(&mut self, owner: &str) {
        self.owner = Some(owner.to_string());
    }
    
    /// Get the culture
    pub fn culture(&self) -> &CultureId {
        &self.culture
    }
    
    /// Get the artifact type
    pub fn artifact_type(&self) -> &ArtifactType {
        &self.artifact_type
    }
    
    /// Get the powers
    pub fn powers(&self) -> &[String] {
        &self.powers
    }
    
    /// Get the creator
    pub fn creator(&self) -> Option<&str> {
        self.creator.as_deref()
    }
    
    /// Get the owner
    pub fn owner(&self) -> Option<&str> {
        self.owner.as_deref()
    }
}

// Trait implementations removed as we're using the enum approach

/// Type of mythological artifact
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ArtifactType {
    Weapon,
    Armor,
    Jewelry,
    Vessel,
    Tool,
    Instrument,
    Clothing,
    Book,
    Symbol,
    Unknown,
    Other(String),
}
