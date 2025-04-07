use serde::{Serialize, Deserialize};
use crate::core::{MythId, Metadata, CultureId, CharacteristicId, EventId};

/// Represents a mythological era or age
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MythologicalEra {
    /// Unique identifier
    pub id: MythId,
    /// Name of the era
    pub name: String,
    /// Description of the era
    pub description: String,
    /// Cultural origin
    pub culture: CultureId,
    /// Order in the sequence of eras
    pub sequence_order: Option<u32>,
    /// Defining characteristics
    pub characteristics: Vec<CharacteristicId>,
    /// How this era ended
    pub end_event: Option<EventId>,
    /// Relationships with other entities
    pub relationships: Vec<MythId>,
    /// Metadata
    pub metadata: Metadata,
}

impl MythologicalEra {
    /// Create a new mythological era
    pub fn new(name: &str, description: &str, culture: &str) -> Self {
        Self {
            id: MythId::new(),
            name: name.to_string(),
            description: description.to_string(),
            culture: CultureId::new(culture),
            sequence_order: None,
            characteristics: Vec::new(),
            end_event: None,
            relationships: Vec::new(),
            metadata: Metadata::new(),
        }
    }
    
    /// Add a characteristic
    pub fn add_characteristic(&mut self, characteristic: &str) {
        self.characteristics.push(CharacteristicId::new(characteristic));
    }
    
    /// Set the sequence order
    pub fn set_sequence_order(&mut self, order: u32) {
        self.sequence_order = Some(order);
    }
    
    /// Set the end event
    pub fn set_end_event(&mut self, event: &str) {
        self.end_event = Some(EventId::new(event));
    }
    
    /// Get the culture
    pub fn culture(&self) -> &CultureId {
        &self.culture
    }
    
    /// Get the sequence order
    pub fn sequence_order(&self) -> Option<u32> {
        self.sequence_order
    }
    
    /// Get the characteristics
    pub fn characteristics(&self) -> &[CharacteristicId] {
        &self.characteristics
    }
    
    /// Get the end event
    pub fn end_event(&self) -> Option<&EventId> {
        self.end_event.as_ref()
    }
}

// Trait implementations removed as we're using the enum approach
