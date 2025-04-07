use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

/// Metadata that can be attached to any mythological entity
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Metadata {
    /// When this entity was created in the ontology
    pub created_at: DateTime<Utc>,
    /// When this entity was last modified in the ontology
    pub updated_at: DateTime<Utc>,
    /// The source of information for this entity
    pub sources: Vec<Source>,
    /// Additional attributes that don't fit elsewhere
    pub attributes: HashMap<String, String>,
    /// Confidence level in the accuracy of this information (0.0-1.0)
    pub confidence: Option<f32>,
}

impl Metadata {
    /// Create new metadata with the current timestamp
    pub fn new() -> Self {
        let now = Utc::now();
        Self {
            created_at: now,
            updated_at: now,
            sources: Vec::new(),
            attributes: HashMap::new(),
            confidence: None,
        }
    }
    
    /// Add a source to this metadata
    pub fn add_source(&mut self, source: Source) {
        self.sources.push(source);
    }
    
    /// Add an attribute to this metadata
    pub fn add_attribute(&mut self, key: &str, value: &str) {
        self.attributes.insert(key.to_string(), value.to_string());
    }
    
    /// Update the timestamp to now
    pub fn update_timestamp(&mut self) {
        self.updated_at = Utc::now();
    }
}

impl Default for Metadata {
    fn default() -> Self {
        Self::new()
    }
}

/// A source of information for mythological data
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Source {
    /// Title of the source
    pub title: String,
    /// Author(s) of the source
    pub author: Option<String>,
    /// Year of publication
    pub year: Option<i32>,
    /// Type of source (book, article, website, etc.)
    pub source_type: SourceType,
    /// URL if available
    pub url: Option<String>,
    /// Additional notes about this source
    pub notes: Option<String>,
}

/// Types of information sources
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SourceType {
    Book,
    Article,
    Website,
    AcademicPaper,
    PrimaryText,
    OralTradition,
    Archaeological,
    CompilationText,   // Collection of texts compiled together
    LiteraryText,      // Literary works with mythological content
    Other(String),
}
