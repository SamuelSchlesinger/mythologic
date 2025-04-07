use serde::{Serialize, Deserialize};
use std::collections::HashSet;
use crate::core::{MythId, Metadata};

/// Represents a mythological creature or monster
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Creature {
    /// Unique identifier
    pub id: MythId,
    /// Primary name
    pub name: String,
    /// Description of the creature
    pub description: String,
    /// Cultural origin
    pub culture: String,
    /// Type of creature
    pub creature_type: CreatureType,
    /// Habitat or dwelling place
    pub habitat: HashSet<String>,
    /// Special abilities or powers
    pub abilities: Vec<String>,
    /// Relationships with other entities
    pub relationships: Vec<MythId>,
    /// Metadata
    pub metadata: Metadata,
}

impl Creature {
    /// Create a new creature
    pub fn new(name: &str, description: &str, culture: &str) -> Self {
        Self {
            id: MythId::new(),
            name: name.to_string(),
            description: description.to_string(),
            culture: culture.to_string(),
            creature_type: CreatureType::Unknown,
            habitat: HashSet::new(),
            abilities: Vec::new(),
            relationships: Vec::new(),
            metadata: Metadata::new(),
        }
    }
    
    /// Add a habitat
    pub fn add_habitat(&mut self, habitat: &str) {
        self.habitat.insert(habitat.to_string());
    }
    
    /// Add an ability
    pub fn add_ability(&mut self, ability: &str) {
        self.abilities.push(ability.to_string());
    }
    
    /// Set the creature type
    pub fn set_creature_type(&mut self, creature_type: CreatureType) {
        self.creature_type = creature_type;
    }
    
    /// Get the culture
    pub fn culture(&self) -> &str {
        &self.culture
    }
    
    /// Get the creature type
    pub fn creature_type(&self) -> &CreatureType {
        &self.creature_type
    }
    
    /// Get the habitats
    pub fn habitats(&self) -> &HashSet<String> {
        &self.habitat
    }
    
    /// Get the abilities
    pub fn abilities(&self) -> &[String] {
        &self.abilities
    }
}

impl MythEntity for Creature {
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
        "Creature"
    }
}

impl Relatable for Creature {
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

/// Type of mythological creature
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CreatureType {
    Dragon,
    Giant,
    Spirit,
    Undead,
    Shapeshifter,
    Hybrid,
    Guardian,
    Monster,
    Elemental,
    Fae,
    Unknown,
    Other(String),
}
