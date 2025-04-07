use serde::{Serialize, Deserialize};
use std::collections::HashSet;
use crate::core::{MythId, Metadata, MythEntity, Relatable};

/// Represents a pantheon of deities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pantheon {
    /// Unique identifier
    id: MythId,
    /// Name of the pantheon
    name: String,
    /// Description of the pantheon
    description: String,
    /// Cultural origin
    culture: String,
    /// Primary deities in this pantheon
    primary_deities: HashSet<MythId>,
    /// Secondary deities in this pantheon
    secondary_deities: HashSet<MythId>,
    /// Cosmological structure
    cosmology: Option<String>,
    /// Relationships with other entities
    relationships: Vec<MythId>,
    /// Metadata
    metadata: Metadata,
}

impl Pantheon {
    /// Create a new pantheon
    pub fn new(name: &str, description: &str, culture: &str) -> Self {
        Self {
            id: MythId::new(),
            name: name.to_string(),
            description: description.to_string(),
            culture: culture.to_string(),
            primary_deities: HashSet::new(),
            secondary_deities: HashSet::new(),
            cosmology: None,
            relationships: Vec::new(),
            metadata: Metadata::new(),
        }
    }
    
    /// Add a primary deity
    pub fn add_primary_deity(&mut self, deity_id: MythId) {
        self.primary_deities.insert(deity_id);
    }
    
    /// Add a secondary deity
    pub fn add_secondary_deity(&mut self, deity_id: MythId) {
        self.secondary_deities.insert(deity_id);
    }
    
    /// Set the cosmology
    pub fn set_cosmology(&mut self, cosmology: &str) {
        self.cosmology = Some(cosmology.to_string());
    }
    
    /// Get the culture
    pub fn culture(&self) -> &str {
        &self.culture
    }
    
    /// Get the primary deities
    pub fn primary_deities(&self) -> &HashSet<MythId> {
        &self.primary_deities
    }
    
    /// Get the secondary deities
    pub fn secondary_deities(&self) -> &HashSet<MythId> {
        &self.secondary_deities
    }
    
    /// Get the cosmology
    pub fn cosmology(&self) -> Option<&str> {
        self.cosmology.as_deref()
    }
    
    /// Check if a deity is part of this pantheon
    pub fn contains_deity(&self, deity_id: &MythId) -> bool {
        self.primary_deities.contains(deity_id) || self.secondary_deities.contains(deity_id)
    }
}

impl MythEntity for Pantheon {
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
        "Pantheon"
    }
}

impl Relatable for Pantheon {
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
