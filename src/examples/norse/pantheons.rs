use crate::core::{MythOntology, MythEntity};
use crate::cultural::Pantheon;

/// Add Norse pantheons to the ontology
pub fn add_norse_pantheons(ontology: &mut MythOntology) {
    // Find entity IDs
    let odin_id = find_entity_id(ontology, "Odin");
    let thor_id = find_entity_id(ontology, "Thor");
    let freyja_id = find_entity_id(ontology, "Freyja");
    let _loki_id = find_entity_id(ontology, "Loki");
    let heimdall_id = find_entity_id(ontology, "Heimdall");
    
    // Create Aesir pantheon
    let mut aesir_pantheon = Pantheon::new(
        "Aesir Pantheon",
        "The principal pantheon of gods in Norse mythology, associated with war, power, and governance.",
        "Norse"
    );
    
    // Add primary deities to the pantheon
    if let Some(odin_id) = &odin_id {
        aesir_pantheon.add_primary_deity(odin_id.clone());
    }
    
    if let Some(thor_id) = &thor_id {
        aesir_pantheon.add_primary_deity(thor_id.clone());
    }
    
    if let Some(heimdall_id) = &heimdall_id {
        aesir_pantheon.add_primary_deity(heimdall_id.clone());
    }
    
    // Set cosmology
    aesir_pantheon.set_cosmology(
        "The Norse cosmos is structured around Yggdrasil, the World Tree, which connects the nine worlds. \
        These include Asgard (home of the Aesir), Vanaheim (home of the Vanir), Alfheim (land of the light elves), \
        Midgard (human realm), Jotunheim (land of giants), Svartalfheim (land of dwarves and dark elves), \
        Niflheim (world of ice and mist), Muspelheim (world of fire), and Helheim (realm of the dead)."
    );
    
    // Add founding myth
    aesir_pantheon.set_founding_myth(
        "After the primordial void of Ginnungagap, the worlds were formed from the body of the first giant, Ymir, \
        who was slain by Odin and his brothers. From Ymir's flesh they made the earth (Midgard), \
        from his blood the seas and lakes, from his bones the mountains, and from his skull the sky."
    );
    
    // Create Vanir pantheon
    let mut vanir_pantheon = Pantheon::new(
        "Vanir Pantheon",
        "The secondary pantheon of gods in Norse mythology, associated with fertility, prosperity, and natural forces.",
        "Norse"
    );
    
    // Add primary deities to the pantheon
    if let Some(freyja_id) = &freyja_id {
        vanir_pantheon.add_primary_deity(freyja_id.clone());
    }
    
    // Set cosmology (same as Aesir)
    vanir_pantheon.set_cosmology(
        "The Vanir share the same cosmic structure as the Aesir, with Yggdrasil connecting the nine worlds. \
        The Vanir primarily inhabit Vanaheim, one of these nine worlds."
    );
    
    // Add founding myth
    vanir_pantheon.set_founding_myth(
        "The Aesir and Vanir once fought a war. After reaching a stalemate, they exchanged hostages \
        to maintain peace. The Vanir sent Njord, Freyr, and Freyja to the Aesir, while the Aesir sent Hoenir \
        and Mimir to the Vanir."
    );
    
    // Add to ontology
    ontology.add_entity(MythEntity::Pantheon(aesir_pantheon));
    ontology.add_entity(MythEntity::Pantheon(vanir_pantheon));
}

/// Helper function to find an entity ID by name
fn find_entity_id(ontology: &MythOntology, name: &str) -> Option<crate::core::MythId> {
    ontology.all_entities().iter()
        .find(|entity| entity.name() == name)
        .map(|entity| entity.id().clone())
}