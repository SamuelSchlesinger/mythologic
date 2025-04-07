//! # Deity Entity Model
//!
//! This module provides types for representing deities (gods and goddesses)
//! from various mythological traditions. It includes the main `Deity` struct,
//! helper types like `DeityName` for alternative names and epithets, and
//! enums for classifying deity attributes like gender and importance.

use serde::{Serialize, Deserialize};
use std::collections::HashSet;
use crate::core::{MythId, Metadata, CultureId, PantheonId, DomainId};

/// A type-safe representation of a deity name or epithet.
///
/// Deities in many mythological traditions have multiple names, titles, 
/// and epithets (descriptive terms that characterize them). This structure
/// provides a type-safe wrapper around these alternative names.
///
/// # Examples
///
/// ```
/// use mythologic::entities::DeityName;
///
/// let name = DeityName::new("The Thunderer");
/// assert_eq!(name.value(), "The Thunderer");
///
/// // Can be created from string literals
/// let name: DeityName = "Sky Father".into();
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DeityName(String);

impl DeityName {
    /// Create a new deity name or epithet.
    ///
    /// # Arguments
    ///
    /// * `name` - The name or epithet as a string slice
    ///
    /// # Returns
    ///
    /// A new `DeityName` instance wrapping the provided string.
    pub fn new(name: &str) -> Self {
        Self(name.to_string())
    }
    
    /// Get the name as a string reference.
    ///
    /// # Returns
    ///
    /// A string slice representing the name.
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl From<&str> for DeityName {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

impl From<String> for DeityName {
    fn from(s: String) -> Self {
        Self(s)
    }
}

/// Represents a deity or god/goddess in mythology.
///
/// The `Deity` struct models divine figures from mythological traditions,
/// capturing their attributes, domains of influence, cultural context, and
/// relationships with other mythological entities.
///
/// # Examples
///
/// ```
/// use mythologic::entities::{Deity, Gender, DeityImportance};
///
/// // Create Zeus, the Greek king of gods
/// let mut zeus = Deity::new("Zeus", "King of the Olympian gods", "Greek");
///
/// // Add details
/// zeus.add_domain("Sky");
/// zeus.add_domain("Thunder");
/// zeus.add_alternative_name("Jupiter"); // Roman equivalent
/// zeus.set_gender(Gender::Male);
/// zeus.set_importance(DeityImportance::Supreme);
/// zeus.set_pantheon("Olympian");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deity {
    /// Unique identifier for this deity
    pub id: MythId,
    /// Primary name by which this deity is known
    pub name: String,
    /// Alternative names, titles, or epithets
    pub alternative_names: Vec<DeityName>,
    /// Description of the deity's nature, history, and significance
    pub description: String,
    /// Spheres of influence or power (e.g., war, love, wisdom)
    pub domains: HashSet<DomainId>,
    /// Cultural tradition to which this deity belongs
    pub culture: CultureId,
    /// Group of related deities (e.g., Olympian, Aesir)
    pub pantheon: Option<PantheonId>,
    /// Gender representation or aspect
    pub gender: Gender,
    /// Relative importance within their pantheon
    pub importance: DeityImportance,
    /// Connections to other mythological entities
    pub relationships: Vec<MythId>,
    /// Additional extensible data 
    pub metadata: Metadata,
}

impl Deity {
    /// Create a new deity with basic information.
    ///
    /// # Arguments
    ///
    /// * `name` - The primary name of the deity
    /// * `description` - A description of the deity
    /// * `culture` - The cultural context (e.g., "Greek", "Norse")
    ///
    /// # Returns
    ///
    /// A new `Deity` instance with default values for optional fields.
    ///
    /// # Examples
    ///
    /// ```
    /// use mythologic::entities::Deity;
    ///
    /// let apollo = Deity::new(
    ///     "Apollo", 
    ///     "God of music, arts, knowledge, healing, plague, prophecy, poetry, and archery", 
    ///     "Greek"
    /// );
    /// ```
    pub fn new(name: &str, description: &str, culture: &str) -> Self {
        Self {
            id: MythId::new(),
            name: name.to_string(),
            alternative_names: Vec::new(),
            description: description.to_string(),
            domains: HashSet::new(),
            culture: CultureId::new(culture),
            pantheon: None,
            gender: Gender::Unknown,
            importance: DeityImportance::Unknown,
            relationships: Vec::new(),
            metadata: Metadata::new(),
        }
    }
    
    /// Add a domain of influence to this deity.
    ///
    /// Domains represent areas over which a deity has power or authority,
    /// such as "war", "love", "wisdom", "thunder", etc.
    ///
    /// # Arguments
    ///
    /// * `domain` - The domain to add
    ///
    /// # Examples
    ///
    /// ```
    /// use mythologic::entities::Deity;
    ///
    /// let mut athena = Deity::new("Athena", "Goddess of wisdom and strategic warfare", "Greek");
    /// athena.add_domain("Wisdom");
    /// athena.add_domain("War");
    /// athena.add_domain("Crafts");
    /// ```
    pub fn add_domain(&mut self, domain: &str) {
        self.domains.insert(DomainId::new(domain));
    }
    
    /// Add an alternative name or epithet for this deity.
    ///
    /// Many deities are known by multiple names, titles, or epithets across
    /// different traditions or contexts.
    ///
    /// # Arguments
    ///
    /// * `name` - The alternative name to add
    ///
    /// # Examples
    ///
    /// ```
    /// use mythologic::entities::Deity;
    ///
    /// let mut poseidon = Deity::new("Poseidon", "God of the sea", "Greek");
    /// poseidon.add_alternative_name("Neptune"); // Roman equivalent
    /// poseidon.add_alternative_name("Earth-Shaker"); // Common epithet
    /// ```
    pub fn add_alternative_name(&mut self, name: &str) {
        self.alternative_names.push(DeityName::new(name));
    }
    
    /// Set the pantheon to which this deity belongs.
    ///
    /// A pantheon is a collection of related deities within a mythological tradition.
    ///
    /// # Arguments
    ///
    /// * `pantheon` - The name of the pantheon (e.g., "Olympian", "Aesir")
    ///
    /// # Examples
    ///
    /// ```
    /// use mythologic::entities::Deity;
    ///
    /// let mut thor = Deity::new("Thor", "God of thunder and strength", "Norse");
    /// thor.set_pantheon("Aesir");
    /// ```
    pub fn set_pantheon(&mut self, pantheon: &str) {
        self.pantheon = Some(PantheonId::new(pantheon));
    }
    
    /// Set the gender representation of this deity.
    ///
    /// Deities in different traditions may have various gender representations,
    /// including non-binary or fluid aspects.
    ///
    /// # Arguments
    ///
    /// * `gender` - The gender representation
    ///
    /// # Examples
    ///
    /// ```
    /// use mythologic::entities::{Deity, Gender};
    ///
    /// let mut freya = Deity::new("Freya", "Goddess of love, beauty, and fertility", "Norse");
    /// freya.set_gender(Gender::Female);
    /// ```
    pub fn set_gender(&mut self, gender: Gender) {
        self.gender = gender;
    }
    
    /// Set the importance level of this deity within their pantheon.
    ///
    /// # Arguments
    ///
    /// * `importance` - The level of importance
    ///
    /// # Examples
    ///
    /// ```
    /// use mythologic::entities::{Deity, DeityImportance};
    ///
    /// let mut odin = Deity::new("Odin", "All-Father and chief of the Aesir", "Norse");
    /// odin.set_importance(DeityImportance::Supreme);
    /// ```
    pub fn set_importance(&mut self, importance: DeityImportance) {
        self.importance = importance;
    }
    
    /// Get all domains of influence for this deity.
    ///
    /// # Returns
    ///
    /// A reference to the set of domains.
    pub fn domains(&self) -> &HashSet<DomainId> {
        &self.domains
    }
    
    /// Get all alternative names for this deity.
    ///
    /// # Returns
    ///
    /// A slice of alternative names.
    pub fn alternative_names(&self) -> &[DeityName] {
        &self.alternative_names
    }
    
    /// Get the pantheon to which this deity belongs, if any.
    ///
    /// # Returns
    ///
    /// An optional reference to the pantheon ID.
    pub fn pantheon(&self) -> Option<&PantheonId> {
        self.pantheon.as_ref()
    }
    
    /// Get the cultural origin of this deity.
    ///
    /// # Returns
    ///
    /// A reference to the culture ID.
    pub fn culture(&self) -> &CultureId {
        &self.culture
    }
}

// Trait implementations removed as we're using the enum approach

/// Gender representation of a deity in mythology.
///
/// Mythological traditions often represent deities with different gender aspects,
/// which may be more complex or fluid than simple binary categorizations.
///
/// # Examples
///
/// ```
/// use mythologic::entities::Gender;
///
/// let male = Gender::Male;
/// let fluid = Gender::Fluid;
/// let other = Gender::Other("Dual-natured".to_string());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Gender {
    /// Male gender representation
    Male,
    /// Female gender representation
    Female,
    /// Non-binary gender representation
    NonBinary,
    /// Fluid or changing gender representation
    Fluid,
    /// Both male and female characteristics
    Androgynous,
    /// Gender is not specified or known
    Unknown,
    /// Custom or culture-specific gender concept
    Other(String),
}

/// Importance or hierarchical level of a deity within their pantheon.
///
/// Mythological traditions often have hierarchies of deities with different levels
/// of power, worship, and narrative significance.
///
/// # Examples
///
/// ```
/// use mythologic::entities::DeityImportance;
///
/// let zeus = DeityImportance::Supreme;  // Chief deity
/// let hermes = DeityImportance::Major;  // Important but not supreme
/// let local_river_god = DeityImportance::Local;  // Worshipped in specific regions
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeityImportance {
    /// Chief or head deity of a pantheon
    Supreme,
    /// Significant deity with broad influence
    Major,
    /// Lesser deity with limited domains
    Minor,
    /// Part divine, part mortal
    Demigod,
    /// Worshipped primarily in a specific region
    Local,
    /// Domestic or family deity
    Household,
    /// Importance level is not specified or known
    Unknown,
}
