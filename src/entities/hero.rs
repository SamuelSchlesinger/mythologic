use serde::{Serialize, Deserialize};
use crate::core::{MythId, Metadata, CultureId};

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
    pub culture: CultureId,
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
            culture: CultureId::new(culture),
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
    pub fn culture(&self) -> &CultureId {
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

// Trait implementations removed as we're using the enum approach

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
