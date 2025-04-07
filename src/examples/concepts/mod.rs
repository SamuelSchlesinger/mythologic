//! Mythological concepts from various cultures

use crate::core::{MythOntology, MythEntity, Source, SourceType};
use crate::entities::{Concept, ConceptType};

/// Create an ontology focused on mythological concepts from various cultures
pub fn create_concepts_ontology() -> MythOntology {
    let mut ontology = MythOntology::new();
    
    // Add Greek concepts
    add_greek_concepts(&mut ontology);
    
    // Add Norse concepts
    add_norse_concepts(&mut ontology);
    
    // Add Eastern concepts
    add_eastern_concepts(&mut ontology);
    
    // Add creation concepts
    add_creation_concepts(&mut ontology);
    
    // Add afterlife concepts
    add_afterlife_concepts(&mut ontology);
    
    // Add virtue and vice concepts
    add_virtue_vice_concepts(&mut ontology);
    
    ontology
}

/// Add Greek mythological concepts
fn add_greek_concepts(ontology: &mut MythOntology) {
    let greek_source = Source {
        title: "Greek Philosophical and Mythological Concepts".to_string(),
        author: None,
        year: Some(-400),
        source_type: SourceType::CompilationText,
        url: None,
        notes: Some("Collection of Greek mythological concepts".to_string()),
    };
    
    // Fate (Moirai)
    let mut fate = Concept::new(
        "Fate",
        "The concept of destiny or predetermination, personified by the three Moirai (Fates).",
        "Greek"
    );
    fate.set_concept_type(ConceptType::Fate);
    fate.add_manifestation("Clotho (spinner of life thread)");
    fate.add_manifestation("Lachesis (measurer of life thread)");
    fate.add_manifestation("Atropos (cutter of life thread)");
    
    // Hubris
    let mut hubris = Concept::new(
        "Hubris",
        "Excessive pride, arrogance, or defiance of the gods that leads to downfall.",
        "Greek"
    );
    hubris.set_concept_type(ConceptType::Vice);
    hubris.add_manifestation("Heroes challenging gods");
    hubris.add_manifestation("Mortals claiming divine status");
    hubris.add_manifestation("Refusal to worship or respect gods");
    
    // Xenia
    let mut xenia = Concept::new(
        "Xenia",
        "Sacred rule of hospitality and guest-friendship, protected by Zeus.",
        "Greek"
    );
    xenia.set_concept_type(ConceptType::Virtue);
    xenia.add_manifestation("Offering food and shelter to travelers");
    xenia.add_manifestation("Exchange of gifts between host and guest");
    xenia.add_manifestation("Divine punishment for those who violate xenia");
    
    // Add to ontology with source
    let mut fate_entity = MythEntity::Concept(fate);
    let mut hubris_entity = MythEntity::Concept(hubris);
    let mut xenia_entity = MythEntity::Concept(xenia);
    
    fate_entity.metadata_mut().add_source(greek_source.clone());
    hubris_entity.metadata_mut().add_source(greek_source.clone());
    xenia_entity.metadata_mut().add_source(greek_source);
    
    ontology.add_entity(fate_entity);
    ontology.add_entity(hubris_entity);
    ontology.add_entity(xenia_entity);
}

/// Add Norse mythological concepts
fn add_norse_concepts(ontology: &mut MythOntology) {
    let norse_source = Source {
        title: "Norse Mythological Concepts".to_string(),
        author: None,
        year: Some(1200),
        source_type: SourceType::CompilationText,
        url: None,
        notes: Some("Collection of Norse mythological concepts".to_string()),
    };
    
    // Ragnarök
    let mut ragnarok = Concept::new(
        "Ragnarök",
        "The prophesied doom of the Norse gods and end of the world, followed by its rebirth.",
        "Norse"
    );
    ragnarok.set_concept_type(ConceptType::Cosmology);
    ragnarok.add_manifestation("Death of major gods including Odin, Thor, and Freyr");
    ragnarok.add_manifestation("Natural disasters and cosmic catastrophes");
    ragnarok.add_manifestation("Final battle between gods and jötnar (giants)");
    ragnarok.add_manifestation("World rebirth and repopulation");
    
    // Wyrd/Örlög
    let mut wyrd = Concept::new(
        "Wyrd",
        "The Norse concept of fate or personal destiny that cannot be escaped.",
        "Norse"
    );
    wyrd.set_concept_type(ConceptType::Fate);
    wyrd.add_manifestation("The Norns who weave fate");
    wyrd.add_manifestation("Predetermined events that cannot be avoided");
    wyrd.add_manifestation("Fatalistic acceptance of one's destiny");
    
    // Honor
    let mut honor = Concept::new(
        "Honor",
        "The Norse concept of personal integrity, honesty, and reputation.",
        "Norse"
    );
    honor.set_concept_type(ConceptType::Virtue);
    honor.add_manifestation("Keeping oaths");
    honor.add_manifestation("Maintaining reputation");
    honor.add_manifestation("Facing death bravely");
    
    // Add to ontology with source
    let mut ragnarok_entity = MythEntity::Concept(ragnarok);
    let mut wyrd_entity = MythEntity::Concept(wyrd);
    let mut honor_entity = MythEntity::Concept(honor);
    
    ragnarok_entity.metadata_mut().add_source(norse_source.clone());
    wyrd_entity.metadata_mut().add_source(norse_source.clone());
    honor_entity.metadata_mut().add_source(norse_source);
    
    ontology.add_entity(ragnarok_entity);
    ontology.add_entity(wyrd_entity);
    ontology.add_entity(honor_entity);
}

/// Add Eastern mythological concepts
fn add_eastern_concepts(ontology: &mut MythOntology) {
    let eastern_source = Source {
        title: "Eastern Philosophical and Mythological Concepts".to_string(),
        author: None,
        year: Some(500),
        source_type: SourceType::CompilationText,
        url: None,
        notes: Some("Collection of Eastern mythological concepts".to_string()),
    };
    
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
    
    // Yin-Yang
    let mut yin_yang = Concept::new(
        "Yin-Yang",
        "The concept of complementary, interconnected opposites that create balance in the universe.",
        "Chinese"
    );
    yin_yang.set_concept_type(ConceptType::Cosmology);
    yin_yang.add_manifestation("Light and dark");
    yin_yang.add_manifestation("Masculine and feminine");
    yin_yang.add_manifestation("Active and passive");
    yin_yang.add_manifestation("Constant flow and change");
    
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
    
    // Add to ontology with source
    let mut karma_entity = MythEntity::Concept(karma);
    let mut yin_yang_entity = MythEntity::Concept(yin_yang);
    let mut dharma_entity = MythEntity::Concept(dharma);
    
    karma_entity.metadata_mut().add_source(eastern_source.clone());
    yin_yang_entity.metadata_mut().add_source(eastern_source.clone());
    dharma_entity.metadata_mut().add_source(eastern_source);
    
    ontology.add_entity(karma_entity);
    ontology.add_entity(yin_yang_entity);
    ontology.add_entity(dharma_entity);
}

/// Add creation concepts from various mythologies
fn add_creation_concepts(ontology: &mut MythOntology) {
    let creation_source = Source {
        title: "World Creation Myths".to_string(),
        author: None,
        year: None,
        source_type: SourceType::CompilationText,
        url: None,
        notes: Some("Collection of creation myths from various cultures".to_string()),
    };
    
    // Ex Nihilo
    let mut ex_nihilo = Concept::new(
        "Creation Ex Nihilo",
        "Creation of the universe from nothing by a supreme deity.",
        "Multiple"
    );
    ex_nihilo.set_concept_type(ConceptType::Creation);
    ex_nihilo.add_manifestation("Biblical creation by God");
    ex_nihilo.add_manifestation("Creation through divine word or thought");
    
    // World Egg
    let mut world_egg = Concept::new(
        "World Egg",
        "Creation myth where the universe or world emerges from a cosmic egg.",
        "Multiple"
    );
    world_egg.set_concept_type(ConceptType::Creation);
    world_egg.add_manifestation("Chinese Pangu myth");
    world_egg.add_manifestation("Hindu Hiranyagarbha (golden egg)");
    world_egg.add_manifestation("Finnish Kalevala world egg");
    
    // Primordial Waters
    let mut primordial_waters = Concept::new(
        "Primordial Waters",
        "Creation from a formless, watery chaos that existed before the universe.",
        "Multiple"
    );
    primordial_waters.set_concept_type(ConceptType::Creation);
    primordial_waters.add_manifestation("Egyptian creation from Nun");
    primordial_waters.add_manifestation("Babylonian Tiamat");
    primordial_waters.add_manifestation("Biblical waters of Genesis");
    
    // Add to ontology with source
    let mut ex_nihilo_entity = MythEntity::Concept(ex_nihilo);
    let mut world_egg_entity = MythEntity::Concept(world_egg);
    let mut waters_entity = MythEntity::Concept(primordial_waters);
    
    ex_nihilo_entity.metadata_mut().add_source(creation_source.clone());
    world_egg_entity.metadata_mut().add_source(creation_source.clone());
    waters_entity.metadata_mut().add_source(creation_source);
    
    ontology.add_entity(ex_nihilo_entity);
    ontology.add_entity(world_egg_entity);
    ontology.add_entity(waters_entity);
}

/// Add afterlife concepts from various mythologies
fn add_afterlife_concepts(ontology: &mut MythOntology) {
    let afterlife_source = Source {
        title: "Afterlife Concepts Across Cultures".to_string(),
        author: None,
        year: None,
        source_type: SourceType::CompilationText,
        url: None,
        notes: Some("Collection of afterlife concepts from various mythologies".to_string()),
    };
    
    // Paradise/Heaven
    let mut paradise = Concept::new(
        "Paradise",
        "A blissful afterlife realm for the righteous or worthy.",
        "Multiple"
    );
    paradise.set_concept_type(ConceptType::Afterlife);
    paradise.add_manifestation("Christian Heaven");
    paradise.add_manifestation("Islamic Jannah");
    paradise.add_manifestation("Greek Elysian Fields");
    paradise.add_manifestation("Norse Valhalla");
    
    // Reincarnation
    let mut reincarnation = Concept::new(
        "Reincarnation",
        "The cycle of rebirth where souls are reborn into new bodies after death.",
        "Multiple"
    );
    reincarnation.set_concept_type(ConceptType::Afterlife);
    reincarnation.add_manifestation("Hindu samsara");
    reincarnation.add_manifestation("Buddhist rebirth");
    reincarnation.add_manifestation("Greek metempsychosis");
    
    // Judgment of the Dead
    let mut judgment = Concept::new(
        "Judgment of the Dead",
        "The evaluation of souls after death to determine their fate in the afterlife.",
        "Multiple"
    );
    judgment.set_concept_type(ConceptType::Afterlife);
    judgment.add_manifestation("Egyptian weighing of the heart");
    judgment.add_manifestation("Christian Last Judgment");
    judgment.add_manifestation("Greek judgment by Minos, Rhadamanthus, and Aeacus");
    
    // Add to ontology with source
    let mut paradise_entity = MythEntity::Concept(paradise);
    let mut reincarnation_entity = MythEntity::Concept(reincarnation);
    let mut judgment_entity = MythEntity::Concept(judgment);
    
    paradise_entity.metadata_mut().add_source(afterlife_source.clone());
    reincarnation_entity.metadata_mut().add_source(afterlife_source.clone());
    judgment_entity.metadata_mut().add_source(afterlife_source);
    
    ontology.add_entity(paradise_entity);
    ontology.add_entity(reincarnation_entity);
    ontology.add_entity(judgment_entity);
}

/// Add virtue and vice concepts from various mythologies
fn add_virtue_vice_concepts(ontology: &mut MythOntology) {
    let virtue_vice_source = Source {
        title: "Virtue and Vice in World Mythology".to_string(),
        author: None,
        year: None,
        source_type: SourceType::CompilationText,
        url: None,
        notes: Some("Collection of virtue and vice concepts from various cultures".to_string()),
    };
    
    // Heroic Sacrifice
    let mut sacrifice = Concept::new(
        "Heroic Sacrifice",
        "The willingness to give up one's life or well-being for others or a greater cause.",
        "Multiple"
    );
    sacrifice.set_concept_type(ConceptType::Virtue);
    sacrifice.add_manifestation("Prometheus giving fire to humans");
    sacrifice.add_manifestation("Christ's sacrifice for humanity");
    sacrifice.add_manifestation("Norse warrior's death in battle");
    
    // Divine Justice
    let mut justice = Concept::new(
        "Divine Justice",
        "The concept that the gods enforce moral order and punish wrongdoing.",
        "Multiple"
    );
    justice.set_concept_type(ConceptType::Justice);
    justice.add_manifestation("Zeus enforcing oaths and hospitality");
    justice.add_manifestation("Hindu karma");
    justice.add_manifestation("Egyptian Ma'at (cosmic order and justice)");
    
    // Cosmic Balance
    let mut balance = Concept::new(
        "Cosmic Balance",
        "The necessity of maintaining equilibrium in the universe between opposing forces.",
        "Multiple"
    );
    balance.set_concept_type(ConceptType::Cosmology);
    balance.add_manifestation("Chinese Yin-Yang");
    balance.add_manifestation("Hindu Trimurti (creation, preservation, destruction)");
    balance.add_manifestation("Greek concept of moderation (sophrosyne)");
    
    // Add to ontology with source
    let mut sacrifice_entity = MythEntity::Concept(sacrifice);
    let mut justice_entity = MythEntity::Concept(justice);
    let mut balance_entity = MythEntity::Concept(balance);
    
    sacrifice_entity.metadata_mut().add_source(virtue_vice_source.clone());
    justice_entity.metadata_mut().add_source(virtue_vice_source.clone());
    balance_entity.metadata_mut().add_source(virtue_vice_source);
    
    ontology.add_entity(sacrifice_entity);
    ontology.add_entity(justice_entity);
    ontology.add_entity(balance_entity);
}