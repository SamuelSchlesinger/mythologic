//! Norse mythology example ontology

mod entities;
mod relationships;
mod pantheons;
mod culture;

use crate::core::MythOntology;

/// Create a comprehensive Norse mythology ontology including deities, heroes, locations,
/// artifacts, creatures, and concepts, with relationships between them.
pub fn create_norse_ontology() -> MythOntology {
    let mut ontology = MythOntology::new();
    
    // Add all entities, relationships, and cultural context
    entities::add_norse_entities(&mut ontology);
    relationships::add_norse_relationships(&mut ontology);
    pantheons::add_norse_pantheons(&mut ontology);
    culture::add_norse_culture(&mut ontology);
    
    ontology
}