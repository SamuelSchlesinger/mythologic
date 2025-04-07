use serde::{Serialize, Deserialize};
use crate::core::{MythId, Metadata, MythEntity, Relatable};

/// Represents a mythological artifact or object of power
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Artifact {
    /// Unique identifier
    id: MythId,
    /// Primary name
    name: String,
    /// Description of the artifact
    description: String,
    /// Cultural origin
    culture: String,
    /// Type of artifact
    artifact_type: ArtifactType,
    /// Powers or properties
    powers: Vec<String>,
    /// Creator of the artifact
    creator: Option<String>,
    /// Current or last known owner
    owner: Option<String>,
    /// Relationships with other entities
    relationships: Vec<MythId>,
    /// Metadata
    metadata: Metadata,
}

impl Artifact {
    /// Create a new artifact
    pub fn new(name: &str, description: &str, culture: &str) -> Self {
        Self {
            id: MythId::new(),
            name: name.to_string(),
            description: description.to_string(),
            culture: culture.to_string(),
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
    pub fn culture(&self) -> &str {
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

impl MythEntity for Artifact {
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
        "Artifact"
    }
}

impl Relatable for Artifact {
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
