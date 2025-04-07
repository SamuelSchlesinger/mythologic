//! Greek mythology example ontology

mod entities;
mod relationships;
mod pantheons;
mod culture;

use crate::core::MythOntology;

/// Create a comprehensive Greek mythology ontology including deities, heroes, locations,
/// artifacts, creatures, and concepts, with relationships between them.
pub fn create_greek_ontology() -> MythOntology {
    let mut ontology = MythOntology::new();
    
    // Add all entities, relationships, and cultural context
    entities::add_greek_entities(&mut ontology);
    relationships::add_greek_relationships(&mut ontology);
    pantheons::add_greek_pantheons(&mut ontology);
    culture::add_greek_culture(&mut ontology);
    
    ontology
}