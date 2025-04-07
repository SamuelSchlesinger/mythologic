//! Relationships between mythological entities
//!
//! This module provides a unified system for modeling various types of relationships
//! between mythological entities, such as family connections, alliances, conflicts,
//! and transformations.
//!
//! ## Architecture
//!
//! The relationship system uses a combination of traits and composition to provide
//! common functionality across different relationship types while allowing for
//! type-specific extensions. Key components include:
//!
//! - [`Relatable`] trait: Common interface for all relationship types
//! - [`Invertible`] trait: For relationship types that have meaningful inverses
//! - [`Properties`] trait: Extensible property system for specialized fields
//! - Builder pattern: Each relationship type has its own builder for fluent creation
//!
//! ## Example Usage
//!
//! ```rust
//! use mythologic::core::MythId;
//! use mythologic::relationships::{FamilyRelationship, FamilyRelationshipType, Relatable};
//!
//! let zeus_id = MythId::new();
//! let athena_id = MythId::new();
//!
//! // Create a family relationship
//! let family_rel = FamilyRelationship::builder(
//!     "Father-Daughter",
//!     "Zeus is Athena's father",
//!     zeus_id.clone(),
//!     athena_id.clone(),
//!     FamilyRelationshipType::Parent
//! )
//! .strength(0.9)  // A strong relationship
//! .build();
//!
//! // Access common methods via the trait
//! assert_eq!(family_rel.source_id(), &zeus_id);
//! assert_eq!(family_rel.target_id(), &athena_id);
//! assert_eq!(family_rel.strength(), 0.9);
//! ```

mod relationship;
mod family;
mod conflict;
mod alliance;
mod transformation;

pub use relationship::*;
pub use family::*;
pub use conflict::*;
pub use alliance::*;
pub use transformation::*;
