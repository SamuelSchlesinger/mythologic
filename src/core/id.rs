use uuid::Uuid;
use std::fmt;
use serde::{Serialize, Deserialize};

/// A unique identifier for any entity in the mythological ontology
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MythId(Uuid);

impl MythId {
    /// Create a new random MythId
    pub fn new() -> Self {
        MythId(Uuid::new_v4())
    }
    
    /// Create a MythId from a string representation
    pub fn from_str(s: &str) -> Result<Self, uuid::Error> {
        Ok(MythId(Uuid::parse_str(s)?))
    }
    
    /// Get the string representation of this MythId
    pub fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl fmt::Display for MythId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Default for MythId {
    fn default() -> Self {
        Self::new()
    }
}
