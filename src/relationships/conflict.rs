use serde::{Serialize, Deserialize};
use crate::core::{MythId, Metadata, MythEntity};
use crate::relationships::Relationship;

/// Represents a conflict relationship between mythological entities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictRelationship {
    /// Base relationship
    relationship: Relationship,
    /// Type of conflict
    conflict_type: ConflictType,
    /// Outcome of the conflict, if resolved
    outcome: Option<ConflictOutcome>,
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
        let mut relationship = Relationship::new(
            name, 
            description, 
            source_id, 
            target_id, 
            crate::relationships::RelationshipType::Conflict
        );
        
        // Most conflicts are bidirectional
        relationship.set_bidirectional(true);
        
        Self {
            relationship,
            conflict_type,
            outcome: None,
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
}

impl MythEntity for ConflictRelationship {
    fn id(&self) -> &MythId {
        self.relationship.id()
    }
    
    fn name(&self) -> &str {
        self.relationship.name()
    }
    
    fn metadata(&self) -> &Metadata {
        self.relationship.metadata()
    }
    
    fn metadata_mut(&mut self) -> &mut Metadata {
        self.relationship.metadata_mut()
    }
    
    fn entity_type(&self) -> &'static str {
        "ConflictRelationship"
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
