//! Hindu mythology example ontology

use crate::core::{MythOntology, MythEntity, Source, SourceType};
use crate::entities::{Deity, Hero, Location, Concept};
use crate::entities::{Gender, DeityImportance, HeroOrigin, LocationType, ConceptType};
use crate::cultural::{Pantheon, Culture, TimePeriod};

/// Create a comprehensive Hindu mythology ontology
pub fn create_hindu_ontology() -> MythOntology {
    let mut ontology = MythOntology::new();
    
    // Add Hindu deities
    add_hindu_deities(&mut ontology);
    
    // Add Hindu heroes
    add_hindu_heroes(&mut ontology);
    
    // Add Hindu locations
    add_hindu_locations(&mut ontology);
    
    // Add Hindu concepts
    add_hindu_concepts(&mut ontology);
    
    // Add Hindu culture
    add_hindu_culture(&mut ontology);
    
    // Add Hindu pantheon
    add_hindu_pantheon(&mut ontology);
    
    ontology
}

/// Add Hindu deities to the ontology
fn add_hindu_deities(ontology: &mut MythOntology) {
    // Primary source for Hindu deities
    let vedic_source = Source {
        title: "Vedas".to_string(),
        author: None,
        year: Some(-1500),
        source_type: SourceType::PrimaryText,
        url: None,
        notes: Some("Ancient Sanskrit texts of Hinduism".to_string()),
    };
    
    let puranic_source = Source {
        title: "Puranas".to_string(),
        author: None,
        year: Some(500),
        source_type: SourceType::PrimaryText,
        url: None,
        notes: Some("Ancient and medieval texts of Hinduism focusing on deity narratives".to_string()),
    };
    
    // Brahma
    let mut brahma = Deity::new(
        "Brahma",
        "The creator god in the Hindu Trimurti, responsible for the creation of the universe.",
        "Hindu"
    );
    brahma.add_domain("Creation");
    brahma.add_domain("Knowledge");
    brahma.add_domain("Vedas");
    brahma.set_gender(Gender::Male);
    brahma.set_importance(DeityImportance::Major);
    brahma.set_pantheon("Trimurti");
    
    // Vishnu
    let mut vishnu = Deity::new(
        "Vishnu",
        "The preserver god in the Hindu Trimurti, who appears in various avatars to restore dharma.",
        "Hindu"
    );
    vishnu.add_domain("Preservation");
    vishnu.add_domain("Protection");
    vishnu.add_domain("Dharma");
    vishnu.set_gender(Gender::Male);
    vishnu.set_importance(DeityImportance::Supreme);
    vishnu.set_pantheon("Trimurti");
    
    // Shiva
    let mut shiva = Deity::new(
        "Shiva",
        "The destroyer god in the Hindu Trimurti, associated with meditation, yoga, and the cosmic dance.",
        "Hindu"
    );
    shiva.add_domain("Destruction");
    shiva.add_domain("Transformation");
    shiva.add_domain("Meditation");
    shiva.add_domain("Dance");
    shiva.set_gender(Gender::Male);
    shiva.set_importance(DeityImportance::Supreme);
    shiva.set_pantheon("Trimurti");
    
    // Devi/Shakti (Parvati)
    let mut devi = Deity::new(
        "Devi",
        "The supreme goddess in Hinduism, manifesting in various forms including Parvati, Durga, and Kali.",
        "Hindu"
    );
    devi.add_domain("Power");
    devi.add_domain("Energy");
    devi.add_domain("Creation");
    devi.add_domain("Protection");
    devi.set_gender(Gender::Female);
    devi.set_importance(DeityImportance::Supreme);
    devi.set_pantheon("Shaktism");
    devi.add_alternative_name("Shakti");
    devi.add_alternative_name("Parvati");
    
    // Ganesha
    let mut ganesha = Deity::new(
        "Ganesha",
        "Elephant-headed god of beginnings, wisdom, and the remover of obstacles.",
        "Hindu"
    );
    ganesha.add_domain("Beginnings");
    ganesha.add_domain("Wisdom");
    ganesha.add_domain("Obstacles");
    ganesha.add_domain("Arts");
    ganesha.set_gender(Gender::Male);
    ganesha.set_importance(DeityImportance::Major);
    ganesha.set_pantheon("Hindu");
    ganesha.add_alternative_name("Ganapati");
    
    // Krishna
    let mut krishna = Deity::new(
        "Krishna",
        "An avatar of Vishnu, divine hero and teacher who delivered the Bhagavad Gita.",
        "Hindu"
    );
    krishna.add_domain("Protection");
    krishna.add_domain("Dharma");
    krishna.add_domain("Divine Love");
    krishna.add_domain("Wisdom");
    krishna.set_gender(Gender::Male);
    krishna.set_importance(DeityImportance::Major);
    krishna.set_pantheon("Vaishnavism");
    
    // Convert to MythEntity enum and add source
    let mut brahma_entity = MythEntity::Deity(brahma);
    let mut vishnu_entity = MythEntity::Deity(vishnu);
    let mut shiva_entity = MythEntity::Deity(shiva);
    let mut devi_entity = MythEntity::Deity(devi);
    let mut ganesha_entity = MythEntity::Deity(ganesha);
    let mut krishna_entity = MythEntity::Deity(krishna);
    
    // Add source to metadata
    brahma_entity.metadata_mut().add_source(vedic_source.clone());
    brahma_entity.metadata_mut().add_source(puranic_source.clone());
    vishnu_entity.metadata_mut().add_source(vedic_source.clone());
    vishnu_entity.metadata_mut().add_source(puranic_source.clone());
    shiva_entity.metadata_mut().add_source(vedic_source.clone());
    shiva_entity.metadata_mut().add_source(puranic_source.clone());
    devi_entity.metadata_mut().add_source(puranic_source.clone());
    ganesha_entity.metadata_mut().add_source(puranic_source.clone());
    krishna_entity.metadata_mut().add_source(puranic_source.clone());
    
    // Add to ontology
    ontology.add_entity(brahma_entity);
    ontology.add_entity(vishnu_entity);
    ontology.add_entity(shiva_entity);
    ontology.add_entity(devi_entity);
    ontology.add_entity(ganesha_entity);
    ontology.add_entity(krishna_entity);
}

/// Add Hindu heroes to the ontology
fn add_hindu_heroes(ontology: &mut MythOntology) {
    let epic_source = Source {
        title: "Mahabharata and Ramayana".to_string(),
        author: None,
        year: Some(-400),
        source_type: SourceType::PrimaryText,
        url: None,
        notes: Some("Ancient Sanskrit epics of India".to_string()),
    };
    
    // Arjuna
    let mut arjuna = Hero::new(
        "Arjuna",
        "Third of the five Pandava brothers and a central hero of the Mahabharata, known for his archery skills and close relationship with Krishna.",
        "Hindu"
    );
    arjuna.set_origin(HeroOrigin::Demigod);
    arjuna.add_achievement("Winning the archery contest for Draupadi's hand");
    arjuna.add_achievement("Receiving the Bhagavad Gita from Krishna");
    arjuna.add_achievement("Defeating many warriors in the Kurukshetra War");
    
    // Rama
    let mut rama = Hero::new(
        "Rama",
        "Seventh avatar of Vishnu and protagonist of the Ramayana, who rescued his wife Sita from the demon king Ravana.",
        "Hindu"
    );
    rama.set_origin(HeroOrigin::Divine);
    rama.add_achievement("Breaking Shiva's bow");
    rama.add_achievement("Defeating the demoness Tataka");
    rama.add_achievement("Building a bridge to Lanka");
    rama.add_achievement("Defeating the demon king Ravana");
    
    // Hanuman
    let mut hanuman = Hero::new(
        "Hanuman",
        "The divine monkey who aided Rama in his quest to rescue Sita from Ravana, known for his strength, devotion, and magical abilities.",
        "Hindu"
    );
    hanuman.set_origin(HeroOrigin::Divine);
    hanuman.add_achievement("Leaping across the ocean to Lanka");
    hanuman.add_achievement("Finding Sita in captivity");
    hanuman.add_achievement("Burning Lanka with his flaming tail");
    hanuman.add_achievement("Carrying an entire mountain of healing herbs");
    
    // Add to ontology with source
    let mut arjuna_entity = MythEntity::Hero(arjuna);
    let mut rama_entity = MythEntity::Hero(rama);
    let mut hanuman_entity = MythEntity::Hero(hanuman);
    
    arjuna_entity.metadata_mut().add_source(epic_source.clone());
    rama_entity.metadata_mut().add_source(epic_source.clone());
    hanuman_entity.metadata_mut().add_source(epic_source);
    
    ontology.add_entity(arjuna_entity);
    ontology.add_entity(rama_entity);
    ontology.add_entity(hanuman_entity);
}

/// Add Hindu locations to the ontology
fn add_hindu_locations(ontology: &mut MythOntology) {
    // Mount Meru
    let mut meru = Location::new(
        "Mount Meru",
        "The sacred five-peaked mountain at the center of the cosmos in Hindu mythology.",
        "Hindu"
    );
    meru.set_location_type(LocationType::Mountain);
    meru.add_characteristic("Central axis of the universe");
    meru.add_characteristic("Five peaks made of precious substances");
    meru.add_characteristic("Home of Brahma and other deities");
    meru.add_accessibility("Divine or spiritual means");
    
    // Svarga
    let mut svarga = Location::new(
        "Svarga",
        "The celestial paradise where the righteous live in joy before returning to Earth in rebirth.",
        "Hindu"
    );
    svarga.set_location_type(LocationType::Heaven);
    svarga.add_characteristic("Ruled by Indra");
    svarga.add_characteristic("Temporary paradise");
    svarga.add_characteristic("Full of pleasures and delights");
    svarga.add_accessibility("Through righteous deeds and karma");
    
    // Naraka
    let mut naraka = Location::new(
        "Naraka",
        "The underworld where souls are punished for their sins before rebirth.",
        "Hindu"
    );
    naraka.set_location_type(LocationType::Underworld);
    naraka.add_characteristic("Multiple levels of punishment");
    naraka.add_characteristic("Temporary, not eternal");
    naraka.add_characteristic("Each level for specific sins");
    naraka.add_accessibility("Through sinful deeds and karma");
    
    // Add to ontology
    ontology.add_entity(MythEntity::Location(meru));
    ontology.add_entity(MythEntity::Location(svarga));
    ontology.add_entity(MythEntity::Location(naraka));
}

/// Add Hindu concepts to the ontology
fn add_hindu_concepts(ontology: &mut MythOntology) {
    // Karma
    let mut karma = Concept::new(
        "Karma",
        "The cosmic principle of cause and effect, where actions in this life affect future lives.",
        "Hindu"
    );
    karma.set_concept_type(ConceptType::Cosmology);
    karma.add_manifestation("Actions determining future rebirth");
    karma.add_manifestation("Cosmic justice system");
    karma.add_manifestation("Accumulated merits and demerits");
    
    // Dharma
    let mut dharma = Concept::new(
        "Dharma",
        "The cosmic order, law, duty, and virtue that maintains balance in existence.",
        "Hindu"
    );
    dharma.set_concept_type(ConceptType::Virtue);
    dharma.add_manifestation("Cosmic law and order");
    dharma.add_manifestation("Individual duties based on caste and stage of life");
    dharma.add_manifestation("Religious and moral law");
    
    // Samsara
    let mut samsara = Concept::new(
        "Samsara",
        "The cycle of death and rebirth that souls undergo until achieving moksha.",
        "Hindu"
    );
    samsara.set_concept_type(ConceptType::Afterlife);
    samsara.add_manifestation("Continuous cycle of reincarnation");
    samsara.add_manifestation("Influenced by karma");
    samsara.add_manifestation("Escape through moksha (liberation)");
    
    // Add to ontology
    ontology.add_entity(MythEntity::Concept(karma));
    ontology.add_entity(MythEntity::Concept(dharma));
    ontology.add_entity(MythEntity::Concept(samsara));
}

/// Add Hindu culture to the ontology
fn add_hindu_culture(ontology: &mut MythOntology) {
    // Create Hindu culture
    let mut hindu_culture = Culture::new(
        "Hindu",
        "The religious and cultural traditions of the Indian subcontinent, one of the world's oldest living religious traditions."
    );
    
    // Add regions
    hindu_culture.add_region("India");
    hindu_culture.add_region("Nepal");
    hindu_culture.add_region("Southeast Asia");
    
    // Add languages
    hindu_culture.add_language("Sanskrit");
    hindu_culture.add_language("Tamil");
    hindu_culture.add_language("Hindi");
    
    // Add time periods
    hindu_culture.add_time_period(TimePeriod {
        name: "Vedic Period".to_string(),
        start_year: Some(-1500),
        end_year: Some(-500),
        description: Some("Period when the Vedas were composed and early Hindu traditions developed".to_string()),
    });
    
    hindu_culture.add_time_period(TimePeriod {
        name: "Epic Period".to_string(),
        start_year: Some(-500),
        end_year: Some(500),
        description: Some("Period of the composition of the Mahabharata, Ramayana, and early Puranas".to_string()),
    });
    
    hindu_culture.add_time_period(TimePeriod {
        name: "Puranic Period".to_string(),
        start_year: Some(500),
        end_year: Some(1500),
        description: Some("Period of compilation of the Puranas and development of devotional traditions".to_string()),
    });
    
    // Add cultural practices
    hindu_culture.add_cultural_practice("Puja (worship)");
    hindu_culture.add_cultural_practice("Yoga and meditation");
    hindu_culture.add_cultural_practice("Pilgrimage");
    hindu_culture.add_cultural_practice("Festival celebrations");
    
    // Add to ontology
    ontology.add_entity(MythEntity::Culture(hindu_culture));
}

/// Add Hindu pantheon to the ontology
fn add_hindu_pantheon(ontology: &mut MythOntology) {
    // Find entity IDs
    let brahma_id = find_entity_id(ontology, "Brahma");
    let vishnu_id = find_entity_id(ontology, "Vishnu");
    let shiva_id = find_entity_id(ontology, "Shiva");
    let devi_id = find_entity_id(ontology, "Devi");
    
    // Create Trimurti pantheon
    let mut trimurti_pantheon = Pantheon::new(
        "Trimurti",
        "The Hindu trinity of Brahma (creator), Vishnu (preserver), and Shiva (destroyer).",
        "Hindu"
    );
    
    // Add primary deities to the pantheon
    if let Some(brahma_id) = &brahma_id {
        trimurti_pantheon.add_primary_deity(brahma_id.clone());
    }
    
    if let Some(vishnu_id) = &vishnu_id {
        trimurti_pantheon.add_primary_deity(vishnu_id.clone());
    }
    
    if let Some(shiva_id) = &shiva_id {
        trimurti_pantheon.add_primary_deity(shiva_id.clone());
    }
    
    // Set cosmology
    trimurti_pantheon.set_cosmology(
        "Hindu cosmology describes the universe as cyclically created and destroyed over vast time periods called \
        yugas. The universe is seen as having multiple realms or lokas, including heavens, hells, and the earthly \
        realm. Mount Meru stands at the center of the cosmos, with the various heavens above and hells below. The \
        universe is maintained by the cosmic functions of the Trimurti: creation (Brahma), preservation (Vishnu), \
        and destruction/transformation (Shiva)."
    );
    
    // Add founding myth
    trimurti_pantheon.set_founding_myth(
        "In Hindu creation myths, the universe begins with the cosmic egg or Hiranyagarbha floating in primordial \
        waters. From this emerged Brahma, who then created the world and living beings. In another version, Vishnu \
        sleeps on the coils of the serpent Shesha floating in cosmic waters, and Brahma emerges from a lotus growing \
        from Vishnu's navel to begin creation. The universe goes through cycles of creation and destruction, with \
        each cycle lasting billions of years."
    );
    
    // Create Shakti pantheon
    let mut shakti_pantheon = Pantheon::new(
        "Shaktism",
        "The tradition within Hinduism that focuses on the worship of Shakti or Devi as the supreme deity.",
        "Hindu"
    );
    
    // Add primary deities to the pantheon
    if let Some(devi_id) = &devi_id {
        shakti_pantheon.add_primary_deity(devi_id.clone());
    }
    
    // Set cosmology
    shakti_pantheon.set_cosmology(
        "In Shakta cosmology, the universe is created, sustained, and pervaded by the divine feminine energy \
        known as Shakti. This primordial energy is personified as the goddess Devi in her various forms. \
        All existence is seen as the manifestation of Shakti, with the masculine divine (often represented by Shiva) \
        being the passive consciousness that is activated by Shakti's energy."
    );
    
    // Add to ontology
    ontology.add_entity(MythEntity::Pantheon(trimurti_pantheon));
    ontology.add_entity(MythEntity::Pantheon(shakti_pantheon));
}

/// Helper function to find an entity ID by name
fn find_entity_id(ontology: &MythOntology, name: &str) -> Option<crate::core::MythId> {
    ontology.all_entities().iter()
        .find(|entity| entity.name() == name)
        .map(|entity| entity.id().clone())
}