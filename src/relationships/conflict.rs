use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use crate::core::MythId;
use crate::relationships::{Relationship, RelationshipType, Relatable, Invertible, Properties, RelationshipBuilder};

/// Represents a conflict relationship between mythological entities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictRelationship {
    /// Base relationship
    pub relationship: Relationship,
    /// Type of conflict
    pub conflict_type: ConflictType,
    /// Outcome of the conflict, if resolved
    pub outcome: Option<ConflictOutcome>,
    /// Additional properties for extensibility
    #[serde(default)]
    pub properties: HashMap<String, String>,
}

impl ConflictRelationship {
    /// Create a new conflict relationship
    pub fn new(
        name: &str, 
        description: &str, 
        source_id: MythId, 
        target_id: MythId, 
        conflict_type: ConflictType
    ) -> Self {
        let relationship = RelationshipBuilder::new(
            name, 
            description, 
            source_id, 
            target_id, 
            RelationshipType::Conflict
        )
        .bidirectional(true)  // Most conflicts are bidirectional
        .build();
        
        Self {
            relationship,
            conflict_type,
            outcome: None,
            properties: HashMap::new(),
        }
    }
    
    /// Get the conflict type
    pub fn conflict_type(&self) -> &ConflictType {
        &self.conflict_type
    }
    
    /// Set the conflict type
    pub fn set_conflict_type(&mut self, conflict_type: ConflictType) {
        self.conflict_type = conflict_type;
    }
    
    /// Get the outcome
    pub fn outcome(&self) -> Option<&ConflictOutcome> {
        self.outcome.as_ref()
    }
    
    /// Set the outcome
    pub fn set_outcome(&mut self, outcome: ConflictOutcome) {
        self.outcome = Some(outcome);
    }
    
    /// Create a builder for configuring a new conflict relationship
    pub fn builder(
        name: &str, 
        description: &str, 
        source_id: MythId, 
        target_id: MythId, 
        conflict_type: ConflictType
    ) -> ConflictRelationshipBuilder {
        ConflictRelationshipBuilder {
            name: name.to_string(),
            description: description.to_string(),
            source_id,
            target_id,
            conflict_type,
            outcome: None,
            bidirectional: true,
            strength: 0.5,
            properties: HashMap::new(),
        }
    }
}

/// Builder for conflict relationships
pub struct ConflictRelationshipBuilder {
    name: String,
    description: String,
    source_id: MythId,
    target_id: MythId,
    conflict_type: ConflictType,
    outcome: Option<ConflictOutcome>,
    bidirectional: bool,
    strength: f32,
    properties: HashMap<String, String>,
}

impl ConflictRelationshipBuilder {
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
    
    /// Set the outcome
    pub fn outcome(mut self, outcome: ConflictOutcome) -> Self {
        self.outcome = Some(outcome);
        self
    }
    
    /// Add a custom property
    pub fn property(mut self, key: &str, value: &str) -> Self {
        self.properties.insert(key.to_string(), value.to_string());
        self
    }
    
    /// Build the final relationship
    pub fn build(self) -> ConflictRelationship {
        let relationship = RelationshipBuilder::new(
            &self.name,
            &self.description,
            self.source_id,
            self.target_id,
            RelationshipType::Conflict
        )
        .bidirectional(self.bidirectional)
        .strength(self.strength)
        .build();
        
        ConflictRelationship {
            relationship,
            conflict_type: self.conflict_type,
            outcome: self.outcome,
            properties: self.properties,
        }
    }
}

impl Relatable for ConflictRelationship {
    fn base(&self) -> &Relationship {
        &self.relationship
    }
    
    fn base_mut(&mut self) -> &mut Relationship {
        &mut self.relationship
    }
}

impl Properties for ConflictRelationship {
    fn get_property(&self, name: &str) -> Option<&str> {
        // Check general properties map
        self.properties.get(name).map(|s| s.as_str())
    }
    
    fn set_property(&mut self, name: &str, value: &str) {
        self.properties.insert(name.to_string(), value.to_string());
    }
}

/// Type of conflict
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConflictType {
    War,
    Battle,
    Duel,
    Rivalry,
    Contest,
    Betrayal,
    Curse,
    Punishment,
    Other(String),
}

impl Invertible for ConflictType {
    /// Check if this type of relationship is typically bidirectional
    fn is_typically_bidirectional(&self) -> bool {
        match self {
            Self::Curse => false,       // Typically one-way
            Self::Punishment => false,  // Typically one-way
            _ => true, // Most conflict types are bidirectional
        }
    }
    
    /// Get the inverse relationship type
    fn inverse(&self) -> Self {
        // Most conflict types are their own inverse
        self.clone()
    }
}

/// Outcome of a conflict
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConflictOutcome {
    /// Description of the outcome
    pub description: String,
    /// ID of the victor, if any
    pub victor_id: Option<MythId>,
    /// Consequences of the conflict
    pub consequences: Vec<String>,
}
