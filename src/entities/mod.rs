//! # Mythological Entities
//!
//! This module provides structured types for representing various mythological entities 
//! across different cultural traditions.
//!
//! ## Entity Types
//!
//! The module includes these main entity types:
//!
//! - [`Deity`]: Gods and goddesses (Zeus, Odin, etc.)
//! - [`Hero`]: Heroic figures (Hercules, Beowulf, etc.) 
//! - [`Creature`]: Mythological beings (dragons, centaurs, etc.)
//! - [`Artifact`]: Magical or significant objects (Excalibur, etc.)
//! - [`Location`]: Mythical places (Olympus, Valhalla, etc.)
//! - [`Concept`]: Abstract ideas (Fate, Time, Justice, etc.)
//!
//! Each entity type has its own attributes, classifications, and relationships
//! with other entities in the mythological framework.

// Entity modules
pub mod deity;
pub mod hero;
pub mod creature;
pub mod artifact;
pub mod location;
pub mod concept;

// Re-export main entity types
pub use deity::Deity;
pub use hero::Hero;
pub use creature::Creature;
pub use artifact::Artifact;
pub use location::Location;
pub use concept::Concept;

// Re-export entity enums
pub use deity::{Gender, DeityImportance, DeityName};
pub use hero::HeroOrigin;
pub use creature::CreatureType;
pub use artifact::ArtifactType;
pub use location::LocationType;
pub use concept::ConceptType;
