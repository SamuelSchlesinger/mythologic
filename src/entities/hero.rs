use serde::{Serialize, Deserialize};
use crate::core::{MythId, Metadata};

/// Represents a mythological hero or protagonist
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hero {
    /// Unique identifier
    pub id: MythId,
    /// Primary name
    pub name: String,
    /// Description of the hero
    pub description: String,
    /// Cultural origin
    pub culture: String,
    /// Whether the hero is divine, semi-divine, or mortal
    pub origin: HeroOrigin,
    /// Notable achievements or quests
    pub achievements: Vec<String>,
    /// Relationships with other entities
    pub relationships: Vec<MythId>,
    /// Metadata
    pub metadata: Metadata,
}

impl Hero {
    /// Create a new hero
    pub fn new(name: &str, description: &str, culture: &str) -> Self {
        Self {
            id: MythId::new(),
            name: name.to_string(),
            description: description.to_string(),
            culture: culture.to_string(),
            origin: HeroOrigin::Unknown,
            achievements: Vec::new(),
            relationships: Vec::new(),
            metadata: Metadata::new(),
        }
    }
    
    /// Add an achievement
    pub fn add_achievement(&mut self, achievement: &str) {
        self.achievements.push(achievement.to_string());
    }
    
    /// Set the origin
    pub fn set_origin(&mut self, origin: HeroOrigin) {
        self.origin = origin;
    }
    
    /// Get the culture
    pub fn culture(&self) -> &str {
        &self.culture
    }
    
    /// Get the origin
    pub fn origin(&self) -> &HeroOrigin {
        &self.origin
    }
    
    /// Get the achievements
    pub fn achievements(&self) -> &[String] {
        &self.achievements
    }
}

impl MythEntity for Hero {
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
        "Hero"
    }
}

impl Relatable for Hero {
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

/// Origin of a hero
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HeroOrigin {
    Divine,         // Fully divine origin
    Demigod,        // Child of a deity and a mortal
    BlessedMortal,  // Mortal with divine blessing
    Mortal,         // Fully mortal
    Transformed,    // Transformed into hero status
    Unknown,
    Other(String),
}
