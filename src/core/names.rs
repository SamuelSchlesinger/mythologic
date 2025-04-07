//! # Type-Safe Identifiers
//!
//! This module provides strongly-typed identifiers for mythological entities,
//! cultural contexts, and attributes. Using typed identifiers instead of raw
//! strings enhances compile-time safety, improves documentation, and enables
//! IDE support for type-directed autocompletion.
//!
//! The system is based on newtypes wrapping `String` values, providing the
//! benefits of strong typing without runtime overhead.

use std::fmt;
use serde::{Serialize, Deserialize};

/// Common trait for all name-based identifiers in the mythological ontology.
///
/// This trait provides a unified interface for creating and accessing
/// strongly-typed string identifiers. All identifier types in this module
/// implement this trait, allowing generic programming over different ID types.
///
/// # Type Parameters
///
/// The implementing type must be:
/// - `Sized`
/// - `Clone`
/// - `fmt::Display` - for string representation 
/// - `fmt::Debug` - for debugging and logging
/// - `PartialEq` and `Eq` - for comparison operations
/// - `std::hash::Hash` - for use in HashMaps and HashSets
pub trait NameId: Sized + Clone + fmt::Display + fmt::Debug + PartialEq + Eq + std::hash::Hash {
    /// Create a new name ID from a string.
    ///
    /// # Arguments
    ///
    /// * `name` - The string value to wrap in this ID type
    ///
    /// # Examples
    ///
    /// ```
    /// use mythologic::core::{NameId, CultureId};
    ///
    /// let id = CultureId::new("Greek");
    /// assert_eq!(id.value(), "Greek");
    /// ```
    fn new(name: &str) -> Self;
    
    /// Get the inner string value of this ID.
    ///
    /// # Returns
    ///
    /// A string slice representing the inner value of this ID.
    ///
    /// # Examples
    ///
    /// ```
    /// use mythologic::core::{NameId, CultureId};
    ///
    /// let id = CultureId::new("Norse");
    /// assert_eq!(id.value(), "Norse");
    /// ```
    fn value(&self) -> &str;
}

/// Macro for defining type-safe name ID structs.
///
/// This macro generates a newtype wrapper around `String` with implementations of:
/// - `Debug`, `Clone`, `PartialEq`, `Eq`, `Hash`, `Serialize`, `Deserialize`
/// - `Display` for string representation
/// - `NameId` trait
/// - `From<&str>` and `From<String>` for convenient conversion
///
/// # Arguments
///
/// * `$name` - The name of the ID struct to create
/// * `$doc` - Documentation string for the struct
macro_rules! define_name_id {
    ($name:ident, $doc:expr) => {
        #[doc = $doc]
        #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
        pub struct $name(String);

        impl $name {
            /// Create a new ID from a string.
            ///
            /// # Arguments
            ///
            /// * `name` - The string value to wrap
            ///
            /// # Returns
            ///
            /// A new instance of this ID type wrapping the provided string.
            pub fn new(name: &str) -> Self {
                Self(name.to_string())
            }

            /// Get the inner string value.
            ///
            /// # Returns
            ///
            /// The inner string value of this ID.
            pub fn value(&self) -> &str {
                &self.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl NameId for $name {
            fn new(name: &str) -> Self {
                Self::new(name)
            }

            fn value(&self) -> &str {
                self.value()
            }
        }

        impl From<&str> for $name {
            fn from(s: &str) -> Self {
                Self::new(s)
            }
        }

        impl From<String> for $name {
            fn from(s: String) -> Self {
                Self(s)
            }
        }
    };
}

//------------------------------------------------------------------------------
// Cultural identifiers
//------------------------------------------------------------------------------

define_name_id!(CultureId, "A type-safe identifier for cultural contexts in mythology.

`CultureId` represents a unique identifier for a cultural framework 
(e.g., \"Greek\", \"Norse\", \"Egyptian\") within which mythological entities exist.

# Examples

```
use mythologic::core::CultureId;

let greek = CultureId::new(\"Greek\");
let norse = CultureId::new(\"Norse\");

assert_ne!(greek, norse);
assert_eq!(greek.value(), \"Greek\");
```");

define_name_id!(RegionId, "A type-safe identifier for geographical regions in mythology.

`RegionId` represents a unique identifier for a geographical area 
(e.g., \"Mount Olympus\", \"Asgard\", \"Underworld\") that has mythological significance.");

define_name_id!(EraId, "A type-safe identifier for historical eras in mythology.

`EraId` represents a unique identifier for a time period 
(e.g., \"Golden Age\", \"Age of Heroes\") within mythological chronology.");

define_name_id!(PantheonId, "A type-safe identifier for pantheons in mythology.

`PantheonId` represents a unique identifier for a collection of deities 
(e.g., \"Olympian\", \"Aesir\", \"Vanir\") that form a coherent group.");

define_name_id!(LanguageId, "A type-safe identifier for languages in mythology.

`LanguageId` represents a unique identifier for a language 
(e.g., \"Ancient Greek\", \"Old Norse\") associated with mythological narratives.");

define_name_id!(CosmologyId, "A type-safe identifier for cosmological systems in mythology.

`CosmologyId` represents a unique identifier for a cosmological framework 
(e.g., \"Norse World Tree\", \"Greek Cosmos\") that explains the structure
of the universe within a mythological context.");

//------------------------------------------------------------------------------
// Entity identifiers
//------------------------------------------------------------------------------

define_name_id!(DeityId, "A type-safe identifier for deities in mythology.

`DeityId` represents a unique identifier for a god or goddess
(e.g., \"Zeus\", \"Odin\", \"Ra\") in mythological narratives.");

define_name_id!(HeroId, "A type-safe identifier for heroes in mythology.

`HeroId` represents a unique identifier for a heroic figure
(e.g., \"Hercules\", \"Beowulf\", \"Gilgamesh\") in mythological narratives.");

define_name_id!(CreatureId, "A type-safe identifier for creatures in mythology.

`CreatureId` represents a unique identifier for a mythological creature
(e.g., \"Minotaur\", \"Phoenix\", \"Dragon\") in mythological narratives.");

define_name_id!(ArtifactId, "A type-safe identifier for artifacts in mythology.

`ArtifactId` represents a unique identifier for a mythological object
(e.g., \"Excalibur\", \"Mjolnir\", \"Golden Fleece\") with special properties.");

define_name_id!(LocationId, "A type-safe identifier for locations in mythology.

`LocationId` represents a unique identifier for a mythologically significant place
(e.g., \"Olympus\", \"Valhalla\", \"Elysium\") in mythological narratives.");

define_name_id!(ConceptId, "A type-safe identifier for abstract concepts in mythology.

`ConceptId` represents a unique identifier for an abstract idea or principle
(e.g., \"Fate\", \"Justice\", \"Creation\") in mythological frameworks.");

//------------------------------------------------------------------------------
// Attribute identifiers
//------------------------------------------------------------------------------

define_name_id!(DomainId, "A type-safe identifier for domains of influence in mythology.

`DomainId` represents a unique identifier for an area of power or influence
(e.g., \"Thunder\", \"Love\", \"Death\") associated with mythological entities.");

define_name_id!(FeatureId, "A type-safe identifier for geographic features in mythology.

`FeatureId` represents a unique identifier for a geographic characteristic
(e.g., \"Mountain\", \"River\", \"Forest\") with mythological significance.");

define_name_id!(CharacteristicId, "A type-safe identifier for characteristics in mythology.

`CharacteristicId` represents a unique identifier for a trait or quality
(e.g., \"Wisdom\", \"Strength\", \"Beauty\") associated with mythological entities.");

define_name_id!(EventId, "A type-safe identifier for historical events in mythology.

`EventId` represents a unique identifier for a significant occurrence
(e.g., \"Trojan War\", \"Ragnarok\", \"Great Flood\") in mythological narratives.");