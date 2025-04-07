use serde::{Serialize, Deserialize};
use crate::core::{MythId, Metadata, MythEntity};
use crate::relationships::Relationship;

/// Represents an alliance relationship between mythological entities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllianceRelationship {
    /// Base relationship
    relationship: Relationship,
    /// Type of alliance
    alliance_type: AllianceType,
    /// Purpose of the alliance
    purpose: String,
    /// Duration of the alliance, if known
    duration: Option<String>,
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
        let mut relationship = Relationship::new(
            name, 
            description, 
            source_id, 
            target_id, 
            crate::relationships::RelationshipType::Alliance
        );
        
        // Most alliances are bidirectional
        relationship.set_bidirectional(true);
        
        Self {
            relationship,
            alliance_type,
            purpose: purpose.to_string(),
            duration: None,
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
}

impl MythEntity for AllianceRelationship {
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
        "AllianceRelationship"
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
    Other(String),
}
