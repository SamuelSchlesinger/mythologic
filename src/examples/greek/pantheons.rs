use crate::core::{MythOntology, MythEntity};
use crate::cultural::Pantheon;

/// Add Greek pantheons to the ontology
pub fn add_greek_pantheons(ontology: &mut MythOntology) {
    // Find entity IDs
    let zeus_id = find_entity_id(ontology, "Zeus");
    let hera_id = find_entity_id(ontology, "Hera");
    let athena_id = find_entity_id(ontology, "Athena");
    let poseidon_id = find_entity_id(ontology, "Poseidon");
    let apollo_id = find_entity_id(ontology, "Apollo");
    
    // Create Olympian pantheon
    let mut olympian_pantheon = Pantheon::new(
        "Olympian Pantheon",
        "The principal deities in ancient Greek religion and mythology, residing atop Mount Olympus.",
        "Greek"
    );
    
    // Add primary deities to the pantheon
    if let Some(zeus_id) = &zeus_id {
        olympian_pantheon.add_primary_deity(zeus_id.clone());
    }
    
    if let Some(hera_id) = &hera_id {
        olympian_pantheon.add_primary_deity(hera_id.clone());
    }
    
    if let Some(athena_id) = &athena_id {
        olympian_pantheon.add_primary_deity(athena_id.clone());
    }
    
    if let Some(poseidon_id) = &poseidon_id {
        olympian_pantheon.add_primary_deity(poseidon_id.clone());
    }
    
    if let Some(apollo_id) = &apollo_id {
        olympian_pantheon.add_primary_deity(apollo_id.clone());
    }
    
    // Set cosmology
    olympian_pantheon.set_cosmology(
        "The cosmos is divided into three realms: the sky (Zeus), the sea (Poseidon), and the underworld (Hades). \
        Mount Olympus serves as the home of the gods, while humans dwell on the earth. \
        After death, souls travel to the underworld for judgment."
    );
    
    // Add founding myth
    olympian_pantheon.set_founding_myth(
        "After the Titanomachy, Zeus and his siblings overthrew their father Cronus and the other Titans. \
        The world was then divided among the three brothers: Zeus took the sky, Poseidon the sea, and Hades the underworld."
    );
    
    // Add to ontology
    ontology.add_entity(MythEntity::Pantheon(olympian_pantheon));
    
    // Could add more pantheons like Chthonic deities, Titans, etc.
}

/// Helper function to find an entity ID by name
fn find_entity_id(ontology: &MythOntology, name: &str) -> Option<crate::core::MythId> {
    ontology.all_entities().iter()
        .find(|entity| entity.name() == name)
        .map(|entity| entity.id().clone())
}