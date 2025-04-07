//! # Mythological Examples
//! 
//! This module provides comprehensive examples of mythological ontologies for various cultural traditions.
//!
//! ## Cultural Examples
//!
//! Complete mythological systems including deities, heroes, locations, etc:
//!
//! - [`greek`]: Greek mythology (Olympians, heroes, locations)
//! - [`norse`]: Norse mythology (Aesir, Vanir, Midgard)
//! - [`egyptian`]: Egyptian mythology (Ra, Osiris, pharaohs)
//! - [`celtic`]: Celtic mythology (Tuatha Dé Danann, etc.)
//! - [`hindu`]: Hindu mythology (Trimurti, avatars, etc.)
//!
//! ## Entity Type Examples
//!
//! Examples focused on specific entity types across mythologies:
//!
//! - [`artifacts`]: Notable mythological artifacts and objects
//! - [`heroes`]: Heroic figures from various traditions
//! - [`creatures`]: Mythological creatures and monsters
//! - [`locations`]: Mythical places and realms
//! - [`concepts`]: Abstract mythological concepts
//!

// Cultural examples
pub mod greek;
pub mod norse;
pub mod egyptian;
pub mod celtic;
pub mod hindu;

// Entity type examples
pub mod artifacts;
pub mod heroes;
pub mod creatures;
pub mod locations;
pub mod concepts;


// Re-export main creation functions
pub use greek::create_greek_ontology;
pub use norse::create_norse_ontology;
pub use egyptian::create_egyptian_ontology;
pub use celtic::create_celtic_ontology;
pub use hindu::create_hindu_ontology;

// Re-export entity type examples
pub use artifacts::create_artifacts_ontology;
pub use heroes::create_heroes_ontology;
pub use creatures::create_creatures_ontology;
pub use locations::create_locations_ontology;
pub use concepts::create_concepts_ontology;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_greek_ontology() {
        let ontology = create_greek_ontology();
        assert!(ontology.entity_count() > 0, "Greek ontology should have entities");
        
        let entities = ontology.all_entities();
        let has_zeus = entities.iter().any(|e| e.name() == "Zeus");
        assert!(has_zeus, "Greek ontology should have Zeus");
        
        let has_olympus = entities.iter().any(|e| e.name() == "Mount Olympus");
        assert!(has_olympus, "Greek ontology should have Mount Olympus");
    }
    
    #[test]
    fn test_norse_ontology() {
        let ontology = create_norse_ontology();
        assert!(ontology.entity_count() > 0, "Norse ontology should have entities");
        
        let entities = ontology.all_entities();
        let has_odin = entities.iter().any(|e| e.name() == "Odin");
        assert!(has_odin, "Norse ontology should have Odin");
        
        let has_asgard = entities.iter().any(|e| e.name() == "Asgard");
        assert!(has_asgard, "Norse ontology should have Asgard");
    }
    
    #[test]
    fn test_egyptian_ontology() {
        let ontology = create_egyptian_ontology();
        assert!(ontology.entity_count() > 0, "Egyptian ontology should have entities");
        
        let entities = ontology.all_entities();
        let has_ra = entities.iter().any(|e| e.name() == "Ra");
        assert!(has_ra, "Egyptian ontology should have Ra");
    }
    
    #[test]
    fn test_artifacts_ontology() {
        let ontology = create_artifacts_ontology();
        assert!(ontology.entity_count() > 0, "Artifacts ontology should have entities");
        
        let entities = ontology.all_entities();
        let has_excalibur = entities.iter().any(|e| e.name() == "Excalibur");
        assert!(has_excalibur, "Artifacts ontology should have Excalibur");
    }
    
    #[test]
    fn test_heroes_ontology() {
        let ontology = create_heroes_ontology();
        assert!(ontology.entity_count() > 0, "Heroes ontology should have entities");
        
        let entities = ontology.all_entities();
        let has_heracles = entities.iter().any(|e| e.name() == "Heracles");
        assert!(has_heracles, "Heroes ontology should have Heracles");
    }
}
