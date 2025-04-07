//! # Mythologic
//! 
//! A comprehensive ontology for modeling global mythological structures, their relationships,
//! and how they interplay across different cultures and time periods.
//!
//! ## Overview
//!
//! The `mythologic` crate provides a type-safe and extensible system for representing
//! mythological concepts from various cultures throughout human history. It enables:
//!
//! - Modeling of mythological entities (deities, heroes, creatures, etc.)
//! - Cultural contexts and historical eras
//! - Complex relationships between entities
//! - Type-safe identifiers for referencing related concepts
//! - Querying and filtering of mythological data
//!
//! ## Architecture
//!
//! The crate is organized into several key modules:
//!
//! - [`core`]: Core abstractions, traits, identifiers, and type definitions
//! - [`entities`]: Concrete mythological entities like deities, heroes, and creatures
//! - [`cultural`]: Cultural contexts such as pantheons, regions, and historical eras
//! - [`relationships`]: Relationships between mythological entities
//! - [`query`]: Query engine for filtering and retrieving mythological data
//! - [`utils`]: Utility functions and helpers
//! - [`examples`]: Comprehensive mythological examples from various cultures
//!
//! ## Type Safety
//!
//! This library uses the Rust type system to provide strong guarantees about data integrity.
//! Instead of using raw strings for identifiers, domain-specific newtype wrappers are used
//! (e.g., [`CultureId`], [`DeityId`], etc.) to ensure that:
//!
//! 1. References are always of the correct type
//! 2. Errors in referencing become compile-time rather than runtime issues
//! 3. Type-directed auto-completion works effectively
//! 4. Domain-specific validation can be added as needed
//!
//! ## Example Usage
//!
//! ```rust
//! use mythologic::core::{MythId, CultureId};
//! use mythologic::entities::{Deity, Gender, DeityImportance};
//!
//! // Create a new deity
//! let mut zeus = Deity::new("Zeus", "King of the Olympian gods", "Greek");
//!
//! // Add details
//! zeus.add_domain("Sky");
//! zeus.add_domain("Thunder");
//! zeus.add_alternative_name("Jupiter");
//! zeus.set_gender(Gender::Male);
//! zeus.set_importance(DeityImportance::Supreme);
//! zeus.set_pantheon("Olympian");
//!
//! // References are type-safe
//! let culture_id: &CultureId = zeus.culture();
//! assert_eq!(culture_id.value(), "Greek");
//! ```
//!
//! For complete examples, check the [`examples`] module which includes:
//!
//! - Complete mythology ontologies for various cultures (Greek, Norse, Egyptian, Celtic, Hindu)
//! - Collections of specific entity types (artifacts, heroes, creatures, locations, concepts)
//! - Interconnected relationships between mythological entities
//!
//!
//! ## Data Model
//!
//! The data model is built around the [`MythEntity`] enum, which provides a unified 
//! interface for working with different kinds of mythological concepts. Each entity
//! has a unique [`MythId`] and can have relationships with other entities.
//!
//! Specialized newtypes like [`CultureId`] provide strong typing for references
//! between entities, ensuring that you can't accidentally use a deity ID where a
//! culture ID is expected.

pub mod core;
pub mod entities;
pub mod relationships;
pub mod cultural;
pub mod query;
pub mod utils;
pub mod examples;

pub use crate::core::*;
