use serde::{Serialize, Deserialize};
use std::collections::HashSet;
use crate::core::{MythId, Metadata, CultureId, CosmologyId};

/// Represents a pantheon of deities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pantheon {
    /// Unique identifier
    pub id: MythId,
    /// Name of the pantheon
    pub name: String,
    /// Description of the pantheon
    pub description: String,
    /// Cultural origin
    pub culture: CultureId,
    /// Primary deities in this pantheon
    pub primary_deities: HashSet<MythId>,
    /// Secondary deities in this pantheon
    pub secondary_deities: HashSet<MythId>,
    /// Cosmological structure
    pub cosmology: Option<CosmologyId>,
    /// Founding myth of the pantheon
    pub founding_myth: Option<String>,
    /// Relationships with other entities
    pub relationships: Vec<MythId>,
    /// Metadata
    pub metadata: Metadata,
}

impl Pantheon {
    /// Create a new pantheon
    pub fn new(name: &str, description: &str, culture: &str) -> Self {
        Self {
            id: MythId::new(),
            name: name.to_string(),
            description: description.to_string(),
            culture: CultureId::new(culture),
            primary_deities: HashSet::new(),
            secondary_deities: HashSet::new(),
            cosmology: None,
            founding_myth: None,
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
        self.cosmology = Some(CosmologyId::new(cosmology));
    }
    
    /// Get the culture
    pub fn culture(&self) -> &CultureId {
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
    pub fn cosmology(&self) -> Option<&CosmologyId> {
        self.cosmology.as_ref()
    }
    
    /// Check if a deity is part of this pantheon
    pub fn contains_deity(&self, deity_id: &MythId) -> bool {
        self.primary_deities.contains(deity_id) || self.secondary_deities.contains(deity_id)
    }
    
    /// Set the founding myth
    pub fn set_founding_myth(&mut self, myth: &str) {
        self.founding_myth = Some(myth.to_string());
    }
    
    /// Get the founding myth
    pub fn founding_myth(&self) -> Option<&str> {
        self.founding_myth.as_deref()
    }
}

// Trait implementations removed as we're using the enum approach
