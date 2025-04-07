//! Egyptian mythology example ontology

use crate::core::{MythOntology, MythEntity, Source, SourceType};
use crate::entities::{Deity, Location, Concept};
use crate::entities::{Gender, DeityImportance, LocationType, ConceptType};
use crate::cultural::{Pantheon, Culture, TimePeriod};
use crate::relationships::{FamilyRelationship, FamilyRelationshipType};

/// Create a comprehensive Egyptian mythology ontology
pub fn create_egyptian_ontology() -> MythOntology {
    let mut ontology = MythOntology::new();
    
    // Add Egyptian deities
    add_egyptian_deities(&mut ontology);
    
    // Add Egyptian concepts
    add_egyptian_concepts(&mut ontology);
    
    // Add Egyptian locations
    add_egyptian_locations(&mut ontology);
    
    // Add Egyptian culture
    add_egyptian_culture(&mut ontology);
    
    // Add Egyptian pantheon
    add_egyptian_pantheon(&mut ontology);
    
    // Add family relationships
    add_family_relationships(&mut ontology);
    
    ontology
}

/// Add Egyptian deities to the ontology
fn add_egyptian_deities(ontology: &mut MythOntology) {
    // Primary source for Egyptian deities
    let egyptian_source = Source {
        title: "Book of the Dead".to_string(),
        author: None,
        year: Some(-1550),
        source_type: SourceType::PrimaryText,
        url: None,
        notes: Some("Ancient Egyptian funerary texts".to_string()),
    };
    
    // Ra - Sun God
    let mut ra = Deity::new(
        "Ra",
        "Sun god and creator deity, often merged with other gods as Amun-Ra or Ra-Horakhty.",
        "Egyptian"
    );
    ra.add_domain("Sun");
    ra.add_domain("Creation");
    ra.add_domain("Light");
    ra.add_domain("Kingship");
    ra.set_gender(Gender::Male);
    ra.set_importance(DeityImportance::Supreme);
    ra.set_pantheon("Heliopolitan");
    ra.add_alternative_name("Re");
    ra.add_alternative_name("Amun-Ra");
    
    // Osiris
    let mut osiris = Deity::new(
        "Osiris",
        "God of the afterlife, death, life, and resurrection. Ruler of the underworld.",
        "Egyptian"
    );
    osiris.add_domain("Afterlife");
    osiris.add_domain("Resurrection");
    osiris.add_domain("Agriculture");
    osiris.add_domain("Fertility");
    osiris.set_gender(Gender::Male);
    osiris.set_importance(DeityImportance::Major);
    osiris.set_pantheon("Heliopolitan");
    
    // Isis
    let mut isis = Deity::new(
        "Isis",
        "Goddess of magic, healing, and protection. Wife of Osiris and mother of Horus.",
        "Egyptian"
    );
    isis.add_domain("Magic");
    isis.add_domain("Healing");
    isis.add_domain("Protection");
    isis.add_domain("Motherhood");
    isis.set_gender(Gender::Female);
    isis.set_importance(DeityImportance::Major);
    isis.set_pantheon("Heliopolitan");
    
    // Horus
    let mut horus = Deity::new(
        "Horus",
        "God of kingship and the sky, often depicted with a falcon head.",
        "Egyptian"
    );
    horus.add_domain("Sky");
    horus.add_domain("Kingship");
    horus.add_domain("Protection");
    horus.add_domain("War");
    horus.set_gender(Gender::Male);
    horus.set_importance(DeityImportance::Major);
    horus.set_pantheon("Heliopolitan");
    
    // Set/Seth
    let mut set = Deity::new(
        "Set",
        "God of chaos, desert, storms, and violence. Murderer of Osiris and enemy of Horus.",
        "Egyptian"
    );
    set.add_domain("Chaos");
    set.add_domain("Desert");
    set.add_domain("Storms");
    set.add_domain("Foreign Lands");
    set.set_gender(Gender::Male);
    set.set_importance(DeityImportance::Major);
    set.set_pantheon("Heliopolitan");
    set.add_alternative_name("Seth");
    
    // Convert to MythEntity enum and add source
    let mut ra_entity = MythEntity::Deity(ra);
    let mut osiris_entity = MythEntity::Deity(osiris);
    let mut isis_entity = MythEntity::Deity(isis);
    let mut horus_entity = MythEntity::Deity(horus);
    let mut set_entity = MythEntity::Deity(set);
    
    // Add source to metadata
    ra_entity.metadata_mut().add_source(egyptian_source.clone());
    osiris_entity.metadata_mut().add_source(egyptian_source.clone());
    isis_entity.metadata_mut().add_source(egyptian_source.clone());
    horus_entity.metadata_mut().add_source(egyptian_source.clone());
    set_entity.metadata_mut().add_source(egyptian_source);
    
    // Add to ontology
    ontology.add_entity(ra_entity);
    ontology.add_entity(osiris_entity);
    ontology.add_entity(isis_entity);
    ontology.add_entity(horus_entity);
    ontology.add_entity(set_entity);
}

/// Add Egyptian concepts to the ontology
fn add_egyptian_concepts(ontology: &mut MythOntology) {
    // Ma'at
    let mut maat = Concept::new(
        "Ma'at",
        "The concept of truth, balance, order, harmony, law, morality, and justice.",
        "Egyptian"
    );
    maat.set_concept_type(ConceptType::Justice);
    maat.add_manifestation("Weighing of the heart ceremony in afterlife judgment");
    maat.add_manifestation("Pharaoh's role in maintaining Ma'at");
    maat.add_manifestation("Personified as the goddess Ma'at");
    
    // Ka
    let mut ka = Concept::new(
        "Ka",
        "The spiritual essence or life force that distinguishes the living from the dead.",
        "Egyptian"
    );
    ka.set_concept_type(ConceptType::Other("Soul Concept".to_string()));
    ka.add_manifestation("Sustained through offerings to the deceased");
    ka.add_manifestation("Created at birth by Khnum");
    ka.add_manifestation("Represented in tomb statues (ka statues)");
    
    // Add to ontology
    ontology.add_entity(MythEntity::Concept(maat));
    ontology.add_entity(MythEntity::Concept(ka));
}

/// Add Egyptian locations to the ontology
fn add_egyptian_locations(ontology: &mut MythOntology) {
    // Duat (Underworld)
    let mut duat = Location::new(
        "Duat",
        "The Egyptian underworld, where souls traveled after death.",
        "Egyptian"
    );
    duat.set_location_type(LocationType::Underworld);
    duat.add_characteristic("Realm of Osiris");
    duat.add_characteristic("Contains the Hall of Two Truths");
    duat.add_characteristic("Dangerous journey with many obstacles");
    duat.add_accessibility("Death");
    duat.add_accessibility("Knowledge from the Book of the Dead");
    
    // Field of Reeds (Aaru)
    let mut aaru = Location::new(
        "Aaru",
        "The heavenly paradise in Egyptian mythology, also known as the Field of Reeds.",
        "Egyptian"
    );
    aaru.set_location_type(LocationType::Afterlife);
    aaru.add_characteristic("Perfect reflection of earthly life");
    aaru.add_characteristic("Abundant food and drink");
    aaru.add_characteristic("Place of eternal happiness");
    aaru.add_accessibility("Success in the judgment of the dead");
    aaru.add_accessibility("Knowledge of spells from Book of the Dead");
    
    // Add to ontology
    ontology.add_entity(MythEntity::Location(duat));
    ontology.add_entity(MythEntity::Location(aaru));
}

/// Add Egyptian culture to the ontology
fn add_egyptian_culture(ontology: &mut MythOntology) {
    // Create Egyptian culture
    let mut egyptian_culture = Culture::new(
        "Ancient Egyptian",
        "The civilization of ancient Egypt, which flourished along the Nile from approximately 3100 BCE to 332 BCE."
    );
    
    // Add regions
    egyptian_culture.add_region("Lower Egypt");
    egyptian_culture.add_region("Upper Egypt");
    egyptian_culture.add_region("Nubia");
    
    // Add languages
    egyptian_culture.add_language("Ancient Egyptian");
    egyptian_culture.add_language("Demotic");
    egyptian_culture.add_language("Coptic");
    
    // Add time periods
    egyptian_culture.add_time_period(TimePeriod {
        name: "Old Kingdom".to_string(),
        start_year: Some(-2686),
        end_year: Some(-2181),
        description: Some("Period of prosperity and stability known for pyramid-building".to_string()),
    });
    
    egyptian_culture.add_time_period(TimePeriod {
        name: "Middle Kingdom".to_string(),
        start_year: Some(-2055),
        end_year: Some(-1650),
        description: Some("Period of reunification and cultural flourishing".to_string()),
    });
    
    egyptian_culture.add_time_period(TimePeriod {
        name: "New Kingdom".to_string(),
        start_year: Some(-1550),
        end_year: Some(-1069),
        description: Some("Period of Egyptian imperial expansion and increased international contacts".to_string()),
    });
    
    // Add cultural practices
    egyptian_culture.add_cultural_practice("Mummification");
    egyptian_culture.add_cultural_practice("Temple worship");
    egyptian_culture.add_cultural_practice("Hieroglyphic writing");
    egyptian_culture.add_cultural_practice("Monument building");
    
    // Add to ontology
    ontology.add_entity(MythEntity::Culture(egyptian_culture));
}

/// Add Egyptian pantheon to the ontology
fn add_egyptian_pantheon(ontology: &mut MythOntology) {
    // Find entity IDs
    let ra_id = find_entity_id(ontology, "Ra");
    let osiris_id = find_entity_id(ontology, "Osiris");
    let isis_id = find_entity_id(ontology, "Isis");
    let horus_id = find_entity_id(ontology, "Horus");
    let set_id = find_entity_id(ontology, "Set");
    
    // Create Heliopolitan pantheon
    let mut heliopolitan_pantheon = Pantheon::new(
        "Heliopolitan Pantheon",
        "The main grouping of Egyptian deities centered around the cult center of Heliopolis.",
        "Egyptian"
    );
    
    // Add primary deities to the pantheon
    if let Some(ra_id) = &ra_id {
        heliopolitan_pantheon.add_primary_deity(ra_id.clone());
    }
    
    if let Some(osiris_id) = &osiris_id {
        heliopolitan_pantheon.add_primary_deity(osiris_id.clone());
    }
    
    if let Some(isis_id) = &isis_id {
        heliopolitan_pantheon.add_primary_deity(isis_id.clone());
    }
    
    if let Some(horus_id) = &horus_id {
        heliopolitan_pantheon.add_primary_deity(horus_id.clone());
    }
    
    if let Some(set_id) = &set_id {
        heliopolitan_pantheon.add_primary_deity(set_id.clone());
    }
    
    // Set cosmology
    heliopolitan_pantheon.set_cosmology(
        "The cosmos emerged from the primordial waters of Nun. The creator deity (Ra, Atum, or Amun-Ra) \
        created himself and then created the other gods. The world is depicted as a flat plane with the \
        sky (goddess Nut) arched over it, and the earth (god Geb) below. The underworld (Duat) lies beneath \
        the earth. The sun god Ra travels across the sky during the day and through the underworld at night \
        in his solar barque."
    );
    
    // Add founding myth
    heliopolitan_pantheon.set_founding_myth(
        "In the beginning, there was only Nun, the primordial waters of chaos. From this emerged a mound \
        upon which the creator god (Ra or Atum) appeared. He created himself, then created the first gods \
        either through masturbation or by spitting. These first gods, Shu and Tefnut, gave birth to Geb (earth) \
        and Nut (sky), who in turn parented Osiris, Isis, Set, and Nephthys."
    );
    
    // Add to ontology
    ontology.add_entity(MythEntity::Pantheon(heliopolitan_pantheon));
}

/// Add family relationships between Egyptian deities
fn add_family_relationships(ontology: &mut MythOntology) {
    // Find entity IDs
    let osiris_id = find_entity_id(ontology, "Osiris");
    let isis_id = find_entity_id(ontology, "Isis");
    let horus_id = find_entity_id(ontology, "Horus");
    let set_id = find_entity_id(ontology, "Set");
    
    // Osiris and Isis are spouses
    if let (Some(osiris_id), Some(isis_id)) = (osiris_id.clone(), isis_id.clone()) {
        let osiris_isis_relationship = FamilyRelationship::new(
            "Marriage of Osiris and Isis",
            "The divine marriage of Osiris and Isis, central to Egyptian mythology.",
            osiris_id.clone(),
            isis_id.clone(),
            FamilyRelationshipType::Spouse
        );
        
        ontology.add_entity(MythEntity::FamilyRelationship(osiris_isis_relationship));
    }
    
    // Osiris and Isis are Horus's parents
    if let (Some(osiris_id), Some(isis_id), Some(horus_id)) = (osiris_id.clone(), isis_id.clone(), horus_id.clone()) {
        let osiris_horus_relationship = FamilyRelationship::new(
            "Osiris fathers Horus",
            "Osiris is the father of Horus, conceived after Osiris's death by Isis.",
            osiris_id,
            horus_id.clone(),
            FamilyRelationshipType::Parent
        );
        
        let isis_horus_relationship = FamilyRelationship::new(
            "Isis mothers Horus",
            "Isis is the mother of Horus, who she conceived with Osiris after his death.",
            isis_id,
            horus_id,
            FamilyRelationshipType::Parent
        );
        
        ontology.add_entity(MythEntity::FamilyRelationship(osiris_horus_relationship));
        ontology.add_entity(MythEntity::FamilyRelationship(isis_horus_relationship));
    }
    
    // Set is Osiris's brother
    if let (Some(set_id), Some(osiris_id)) = (set_id.clone(), osiris_id.clone()) {
        let set_osiris_relationship = FamilyRelationship::new(
            "Set and Osiris brotherhood",
            "Set and Osiris are brothers, sons of Geb and Nut.",
            set_id,
            osiris_id,
            FamilyRelationshipType::Sibling
        );
        
        ontology.add_entity(MythEntity::FamilyRelationship(set_osiris_relationship));
    }
}

/// Helper function to find an entity ID by name
fn find_entity_id(ontology: &MythOntology, name: &str) -> Option<crate::core::MythId> {
    ontology.all_entities().iter()
        .find(|entity| entity.name() == name)
        .map(|entity| entity.id().clone())
}