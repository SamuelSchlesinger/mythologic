use serde::{Serialize, Deserialize};
use crate::core::{MythId, Metadata, MythEntity, Relatable};

/// Represents a mythological era or age
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MythologicalEra {
    /// Unique identifier
    id: MythId,
    /// Name of the era
    name: String,
    /// Description of the era
    description: String,
    /// Cultural origin
    culture: String,
    /// Order in the sequence of eras
    sequence_order: Option<u32>,
    /// Defining characteristics
    characteristics: Vec<String>,
    /// How this era ended
    end_event: Option<String>,
    /// Relationships with other entities
    relationships: Vec<MythId>,
    /// Metadata
    metadata: Metadata,
}

impl MythologicalEra {
    /// Create a new mythological era
    pub fn new(name: &str, description: &str, culture: &str) -> Self {
        Self {
            id: MythId::new(),
            name: name.to_string(),
            description: description.to_string(),
            culture: culture.to_string(),
            sequence_order: None,
            characteristics: Vec::new(),
            end_event: None,
            relationships: Vec::new(),
            metadata: Metadata::new(),
        }
    }
    
    /// Add a characteristic
    pub fn add_characteristic(&mut self, characteristic: &str) {
        self.characteristics.push(characteristic.to_string());
    }
    
    /// Set the sequence order
    pub fn set_sequence_order(&mut self, order: u32) {
        self.sequence_order = Some(order);
    }
    
    /// Set the end event
    pub fn set_end_event(&mut self, event: &str) {
        self.end_event = Some(event.to_string());
    }
    
    /// Get the culture
    pub fn culture(&self) -> &str {
        &self.culture
    }
    
    /// Get the sequence order
    pub fn sequence_order(&self) -> Option<u32> {
        self.sequence_order
    }
    
    /// Get the characteristics
    pub fn characteristics(&self) -> &[String] {
        &self.characteristics
    }
    
    /// Get the end event
    pub fn end_event(&self) -> Option<&str> {
        self.end_event.as_deref()
    }
}

impl MythEntity for MythologicalEra {
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
        "MythologicalEra"
    }
}

impl Relatable for MythologicalEra {
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
