use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use crate::core::MythId;
use crate::relationships::{Relationship, RelationshipType, Relatable, Invertible, Properties, RelationshipBuilder};

/// Represents an alliance relationship between mythological entities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllianceRelationship {
    /// Base relationship
    pub relationship: Relationship,
    /// Type of alliance
    pub alliance_type: AllianceType,
    /// Purpose of the alliance
    pub purpose: String,
    /// Duration of the alliance, if known
    pub duration: Option<String>,
    /// Additional properties for extensibility
    #[serde(default)]
    pub properties: HashMap<String, String>,
}

impl AllianceRelationship {
    /// Create a new alliance relationship
    pub fn new(
        name: &str, 
        description: &str, 
        source_id: MythId, 
        target_id: MythId, 
        alliance_type: AllianceType,
        purpose: &str
    ) -> Self {
        let relationship = RelationshipBuilder::new(
            name, 
            description, 
            source_id, 
            target_id, 
            RelationshipType::Alliance
        )
        .bidirectional(true)  // Most alliances are bidirectional
        .build();
        
        Self {
            relationship,
            alliance_type,
            purpose: purpose.to_string(),
            duration: None,
            properties: HashMap::new(),
        }
    }
    
    /// Get the alliance type
    pub fn alliance_type(&self) -> &AllianceType {
        &self.alliance_type
    }
    
    /// Set the alliance type
    pub fn set_alliance_type(&mut self, alliance_type: AllianceType) {
        self.alliance_type = alliance_type;
    }
    
    /// Get the purpose
    pub fn purpose(&self) -> &str {
        &self.purpose
    }
    
    /// Set the purpose
    pub fn set_purpose(&mut self, purpose: &str) {
        self.purpose = purpose.to_string();
    }
    
    /// Get the duration
    pub fn duration(&self) -> Option<&str> {
        self.duration.as_deref()
    }
    
    /// Set the duration
    pub fn set_duration(&mut self, duration: &str) {
        self.duration = Some(duration.to_string());
    }
    
    /// Create a builder for configuring a new alliance relationship
    pub fn builder(
        name: &str, 
        description: &str, 
        source_id: MythId, 
        target_id: MythId, 
        alliance_type: AllianceType,
        purpose: &str
    ) -> AllianceRelationshipBuilder {
        AllianceRelationshipBuilder {
            name: name.to_string(),
            description: description.to_string(),
            source_id,
            target_id,
            alliance_type,
            purpose: purpose.to_string(),
            duration: None,
            bidirectional: true,
            strength: 0.5,
            properties: HashMap::new(),
        }
    }
}

/// Builder for alliance relationships
pub struct AllianceRelationshipBuilder {
    name: String,
    description: String,
    source_id: MythId,
    target_id: MythId,
    alliance_type: AllianceType,
    purpose: String,
    duration: Option<String>,
    bidirectional: bool,
    strength: f32,
    properties: HashMap<String, String>,
}

impl AllianceRelationshipBuilder {
    /// Set the relationship strength
    pub fn strength(mut self, strength: f32) -> Self {
        self.strength = strength;
        self
    }
    
    /// Set whether the relationship is bidirectional
    pub fn bidirectional(mut self, bidirectional: bool) -> Self {
        self.bidirectional = bidirectional;
        self
    }
    
    /// Set the duration
    pub fn duration(mut self, duration: &str) -> Self {
        self.duration = Some(duration.to_string());
        self
    }
    
    /// Add a custom property
    pub fn property(mut self, key: &str, value: &str) -> Self {
        self.properties.insert(key.to_string(), value.to_string());
        self
    }
    
    /// Build the final relationship
    pub fn build(self) -> AllianceRelationship {
        let relationship = RelationshipBuilder::new(
            &self.name,
            &self.description,
            self.source_id,
            self.target_id,
            RelationshipType::Alliance
        )
        .bidirectional(self.bidirectional)
        .strength(self.strength)
        .build();
        
        AllianceRelationship {
            relationship,
            alliance_type: self.alliance_type,
            purpose: self.purpose,
            duration: self.duration,
            properties: self.properties,
        }
    }
}

impl Relatable for AllianceRelationship {
    fn base(&self) -> &Relationship {
        &self.relationship
    }
    
    fn base_mut(&mut self) -> &mut Relationship {
        &mut self.relationship
    }
}

impl Properties for AllianceRelationship {
    fn get_property(&self, name: &str) -> Option<&str> {
        // First check specialized fields
        match name {
            "purpose" => Some(&self.purpose),
            "duration" => self.duration.as_deref(),
            // Then check the general properties map
            _ => self.properties.get(name).map(|s| s.as_str()),
        }
    }
    
    fn set_property(&mut self, name: &str, value: &str) {
        // Handle specialized fields
        match name {
            "purpose" => self.purpose = value.to_string(),
            "duration" => self.duration = Some(value.to_string()),
            // Otherwise store in the general properties map
            _ => { self.properties.insert(name.to_string(), value.to_string()); }
        }
    }
}

/// Type of alliance
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AllianceType {
    Military,
    Political,
    Marriage,
    Oath,
    Pact,
    Friendship,
    Mentorship,
    Patronage,    // Divine or royal sponsorship/protection
    Coalition,    // Group alliance of multiple entities
    Other(String),
}

impl Invertible for AllianceType {
    /// Check if this type of relationship is typically bidirectional
    fn is_typically_bidirectional(&self) -> bool {
        match self {
            Self::Patronage => false,  // Typically one-way
            Self::Mentorship => false, // Typically one-way
            _ => true, // Most alliance types are bidirectional
        }
    }
    
    /// Get the inverse relationship type
    fn inverse(&self) -> Self {
        // Most alliance types are their own inverse
        self.clone()
    }
}
