use serde::{Serialize, Deserialize};
use std::collections::HashSet;
use crate::core::{MythId, Metadata};

/// Represents a deity or god/goddess in mythology
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deity {
    /// Unique identifier
    pub id: MythId,
    /// Primary name
    pub name: String,
    /// Alternative names or epithets
    pub alternative_names: Vec<String>,
    /// Description of the deity
    pub description: String,
    /// Domains of influence (e.g., war, love, wisdom)
    pub domains: HashSet<String>,
    /// Cultural origin
    pub culture: String,
    /// Pantheon this deity belongs to
    pub pantheon: Option<String>,
    /// Gender of the deity
    pub gender: Gender,
    /// Importance level in their pantheon
    pub importance: DeityImportance,
    /// Relationships with other entities
    pub relationships: Vec<MythId>,
    /// Metadata
    pub metadata: Metadata,
}

impl Deity {
    /// Create a new deity
    pub fn new(name: &str, description: &str, culture: &str) -> Self {
        Self {
            id: MythId::new(),
            name: name.to_string(),
            alternative_names: Vec::new(),
            description: description.to_string(),
            domains: HashSet::new(),
            culture: culture.to_string(),
            pantheon: None,
            gender: Gender::Unknown,
            importance: DeityImportance::Unknown,
            relationships: Vec::new(),
            metadata: Metadata::new(),
        }
    }
    
    /// Add a domain of influence
    pub fn add_domain(&mut self, domain: &str) {
        self.domains.insert(domain.to_string());
    }
    
    /// Add an alternative name
    pub fn add_alternative_name(&mut self, name: &str) {
        self.alternative_names.push(name.to_string());
    }
    
    /// Set the pantheon
    pub fn set_pantheon(&mut self, pantheon: &str) {
        self.pantheon = Some(pantheon.to_string());
    }
    
    /// Set the gender
    pub fn set_gender(&mut self, gender: Gender) {
        self.gender = gender;
    }
    
    /// Set the importance
    pub fn set_importance(&mut self, importance: DeityImportance) {
        self.importance = importance;
    }
    
    /// Get all domains
    pub fn domains(&self) -> &HashSet<String> {
        &self.domains
    }
    
    /// Get alternative names
    pub fn alternative_names(&self) -> &[String] {
        &self.alternative_names
    }
    
    /// Get the pantheon
    pub fn pantheon(&self) -> Option<&str> {
        self.pantheon.as_deref()
    }
    
    /// Get the culture
    pub fn culture(&self) -> &str {
        &self.culture
    }
}

// Trait implementations removed as we're using the enum approach

/// Gender of a deity
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Gender {
    Male,
    Female,
    NonBinary,
    Fluid,
    Androgynous,
    Unknown,
    Other(String),
}

/// Importance level of a deity in their pantheon
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeityImportance {
    Supreme,      // Chief deity
    Major,        // Important deity
    Minor,        // Lesser deity
    Demigod,      // Part deity, part mortal
    Local,        // Worshipped in a specific region
    Household,    // Domestic worship
    Unknown,
}
