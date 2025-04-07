use crate::core::{MythOntology, MythEntity, Source, SourceType};
use crate::entities::{Deity, Hero, Creature, Artifact, Location, Concept};
use crate::entities::{Gender, DeityImportance, HeroOrigin, CreatureType, ArtifactType, LocationType, ConceptType};

/// Add Greek deities, heroes, creatures, artifacts, locations, and concepts to the ontology
pub fn add_greek_entities(ontology: &mut MythOntology) {
    // Add deities
    add_greek_deities(ontology);
    
    // Add heroes
    add_greek_heroes(ontology);
    
    // Add creatures
    add_greek_creatures(ontology);
    
    // Add artifacts
    add_greek_artifacts(ontology);
    
    // Add locations
    add_greek_locations(ontology);
    
    // Add concepts
    add_greek_concepts(ontology);
}

/// Add Greek deities to the ontology
fn add_greek_deities(ontology: &mut MythOntology) {
    // Primary source for Greek deities
    let hesiod_source = Source {
        title: "Theogony".to_string(),
        author: Some("Hesiod".to_string()),
        year: Some(-700),
        source_type: SourceType::PrimaryText,
        url: None,
        notes: Some("Ancient Greek poem describing the origins of the gods".to_string()),
    };
    
    // Zeus - King of the gods
    let mut zeus = Deity::new(
        "Zeus",
        "King of the gods and ruler of Mount Olympus, god of the sky, lightning, thunder, law, order, and justice.",
        "Greek"
    );
    zeus.add_domain("Sky");
    zeus.add_domain("Thunder");
    zeus.add_domain("Lightning");
    zeus.add_domain("Law");
    zeus.add_domain("Justice");
    zeus.set_gender(Gender::Male);
    zeus.set_importance(DeityImportance::Supreme);
    zeus.set_pantheon("Olympian");
    zeus.add_alternative_name("Jupiter"); // Roman equivalent
    
    // Hera - Queen of the gods
    let mut hera = Deity::new(
        "Hera",
        "Queen of the gods and goddess of women, marriage, family, and childbirth.",
        "Greek"
    );
    hera.add_domain("Marriage");
    hera.add_domain("Women");
    hera.add_domain("Family");
    hera.add_domain("Childbirth");
    hera.set_gender(Gender::Female);
    hera.set_importance(DeityImportance::Major);
    hera.set_pantheon("Olympian");
    hera.add_alternative_name("Juno"); // Roman equivalent
    
    // Athena - Goddess of wisdom
    let mut athena = Deity::new(
        "Athena",
        "Goddess of wisdom, courage, inspiration, civilization, law and justice, strategic warfare, mathematics, strength, strategy, the arts, crafts, and skill.",
        "Greek"
    );
    athena.add_domain("Wisdom");
    athena.add_domain("Warfare");
    athena.add_domain("Crafts");
    athena.add_domain("Strategy");
    athena.set_gender(Gender::Female);
    athena.set_importance(DeityImportance::Major);
    athena.set_pantheon("Olympian");
    athena.add_alternative_name("Minerva"); // Roman equivalent
    
    // Poseidon - God of the sea
    let mut poseidon = Deity::new(
        "Poseidon",
        "God of the sea, storms, earthquakes, and horses.",
        "Greek"
    );
    poseidon.add_domain("Sea");
    poseidon.add_domain("Storms");
    poseidon.add_domain("Earthquakes");
    poseidon.add_domain("Horses");
    poseidon.set_gender(Gender::Male);
    poseidon.set_importance(DeityImportance::Major);
    poseidon.set_pantheon("Olympian");
    poseidon.add_alternative_name("Neptune"); // Roman equivalent
    
    // Apollo - God of music, arts, and prophecy
    let mut apollo = Deity::new(
        "Apollo",
        "God of music, arts, knowledge, healing, plague, prophecy, poetry, manly beauty, and archery.",
        "Greek"
    );
    apollo.add_domain("Music");
    apollo.add_domain("Prophecy");
    apollo.add_domain("Healing");
    apollo.add_domain("Archery");
    apollo.add_domain("Sun");
    apollo.set_gender(Gender::Male);
    apollo.set_importance(DeityImportance::Major);
    apollo.set_pantheon("Olympian");
    
    // Convert to MythEntity enum and add source metadata
    let mut zeus_entity = MythEntity::Deity(zeus);
    let mut hera_entity = MythEntity::Deity(hera);
    let mut athena_entity = MythEntity::Deity(athena);
    let mut poseidon_entity = MythEntity::Deity(poseidon);
    let mut apollo_entity = MythEntity::Deity(apollo);
    
    // Add source to metadata
    zeus_entity.metadata_mut().add_source(hesiod_source.clone());
    hera_entity.metadata_mut().add_source(hesiod_source.clone());
    athena_entity.metadata_mut().add_source(hesiod_source.clone());
    poseidon_entity.metadata_mut().add_source(hesiod_source.clone());
    apollo_entity.metadata_mut().add_source(hesiod_source);
    
    // Add to ontology
    ontology.add_entity(zeus_entity);
    ontology.add_entity(hera_entity);
    ontology.add_entity(athena_entity);
    ontology.add_entity(poseidon_entity);
    ontology.add_entity(apollo_entity);
}

/// Add Greek heroes to the ontology
fn add_greek_heroes(ontology: &mut MythOntology) {
    let homer_source = Source {
        title: "Iliad & Odyssey".to_string(),
        author: Some("Homer".to_string()),
        year: Some(-750),
        source_type: SourceType::PrimaryText,
        url: None,
        notes: Some("Ancient Greek epic poems depicting the Trojan War and its aftermath".to_string()),
    };
    
    // Heracles (Hercules)
    let mut heracles = Hero::new(
        "Heracles",
        "Greatest of the Greek heroes, known for his extraordinary strength, courage, and completing the twelve labors.",
        "Greek"
    );
    heracles.set_origin(HeroOrigin::Demigod);
    heracles.add_achievement("Twelve Labors of Heracles");
    heracles.add_achievement("Slaying the Nemean Lion");
    heracles.add_achievement("Capturing the Erymanthian Boar");
    heracles.add_achievement("Cleaning the Augean Stables");
    heracles.add_achievement("Stealing the Apples of the Hesperides");
    
    // Perseus
    let mut perseus = Hero::new(
        "Perseus",
        "Legendary founder of Mycenae and slayer of the Gorgon Medusa.",
        "Greek"
    );
    perseus.set_origin(HeroOrigin::Demigod);
    perseus.add_achievement("Beheading Medusa");
    perseus.add_achievement("Rescuing Andromeda from the sea monster");
    perseus.add_achievement("Using Medusa's head to defeat his enemies");
    
    // Odysseus
    let mut odysseus = Hero::new(
        "Odysseus",
        "King of Ithaca known for his intelligence, cunning, and the ten-year journey home after the Trojan War.",
        "Greek"
    );
    odysseus.set_origin(HeroOrigin::BlessedMortal);
    odysseus.add_achievement("Devising the Trojan Horse");
    odysseus.add_achievement("Blinding the Cyclops Polyphemus");
    odysseus.add_achievement("Resisting the Sirens' song");
    odysseus.add_achievement("Returning to Ithaca and reclaiming his throne");
    
    // Convert to MythEntity and add source
    let mut heracles_entity = MythEntity::Hero(heracles);
    let mut perseus_entity = MythEntity::Hero(perseus);
    let mut odysseus_entity = MythEntity::Hero(odysseus);
    
    // Add source to metadata
    heracles_entity.metadata_mut().add_source(homer_source.clone());
    perseus_entity.metadata_mut().add_source(homer_source.clone());
    odysseus_entity.metadata_mut().add_source(homer_source);
    
    // Add to ontology
    ontology.add_entity(heracles_entity);
    ontology.add_entity(perseus_entity);
    ontology.add_entity(odysseus_entity);
}

/// Add Greek creatures to the ontology
fn add_greek_creatures(ontology: &mut MythOntology) {
    // Minotaur
    let mut minotaur = Creature::new(
        "Minotaur",
        "Creature with the head of a bull and the body of a man, kept in the Labyrinth of Crete.",
        "Greek"
    );
    minotaur.set_creature_type(CreatureType::Hybrid);
    minotaur.add_habitat("Labyrinth");
    minotaur.add_habitat("Crete");
    minotaur.add_ability("Superhuman strength");
    
    // Cerberus
    let mut cerberus = Creature::new(
        "Cerberus",
        "Three-headed dog that guards the entrance to the Underworld.",
        "Greek"
    );
    cerberus.set_creature_type(CreatureType::Guardian);
    cerberus.add_habitat("Underworld");
    cerberus.add_ability("Multiple heads");
    cerberus.add_ability("Supernatural senses");
    cerberus.add_ability("Preventing souls from escaping the Underworld");
    
    // Medusa
    let mut medusa = Creature::new(
        "Medusa",
        "One of the Gorgon sisters with snakes for hair, whose gaze turns people to stone.",
        "Greek"
    );
    medusa.set_creature_type(CreatureType::Monster);
    medusa.add_habitat("Island of the Gorgons");
    medusa.add_ability("Petrifying gaze");
    medusa.add_ability("Snake hair");
    
    // Convert to MythEntity
    let minotaur_entity = MythEntity::Creature(minotaur);
    let cerberus_entity = MythEntity::Creature(cerberus);
    let medusa_entity = MythEntity::Creature(medusa);
    
    // Add to ontology
    ontology.add_entity(minotaur_entity);
    ontology.add_entity(cerberus_entity);
    ontology.add_entity(medusa_entity);
}

/// Add Greek artifacts to the ontology
fn add_greek_artifacts(ontology: &mut MythOntology) {
    // Zeus's Thunderbolt
    let mut thunderbolt = Artifact::new(
        "Thunderbolt",
        "Zeus's primary weapon, able to create lightning and thunder.",
        "Greek"
    );
    thunderbolt.set_artifact_type(ArtifactType::Weapon);
    thunderbolt.add_power("Creates lightning");
    thunderbolt.add_power("Produces thunder");
    thunderbolt.add_power("Destroys targets with divine power");
    thunderbolt.set_creator("Cyclopes");
    thunderbolt.set_owner("Zeus");
    
    // Aegis
    let mut aegis = Artifact::new(
        "Aegis",
        "A shield or breastplate associated with Zeus and Athena, often bearing the head of Medusa.",
        "Greek"
    );
    aegis.set_artifact_type(ArtifactType::Armor);
    aegis.add_power("Divine protection");
    aegis.add_power("Creates fear in enemies");
    aegis.set_creator("Hephaestus");
    
    // Golden Fleece
    let mut golden_fleece = Artifact::new(
        "Golden Fleece",
        "The gold-haired winged ram's fleece sought by Jason and the Argonauts.",
        "Greek"
    );
    golden_fleece.set_artifact_type(ArtifactType::Symbol);
    golden_fleece.add_power("Healing properties");
    golden_fleece.add_power("Symbol of authority and kingship");
    golden_fleece.set_owner("King Aeëtes (initially)");
    
    // Convert to MythEntity
    let thunderbolt_entity = MythEntity::Artifact(thunderbolt);
    let aegis_entity = MythEntity::Artifact(aegis);
    let golden_fleece_entity = MythEntity::Artifact(golden_fleece);
    
    // Add to ontology
    ontology.add_entity(thunderbolt_entity);
    ontology.add_entity(aegis_entity);
    ontology.add_entity(golden_fleece_entity);
}

/// Add Greek locations to the ontology
fn add_greek_locations(ontology: &mut MythOntology) {
    // Mount Olympus
    let mut olympus = Location::new(
        "Mount Olympus",
        "Home of the Olympian gods, tallest mountain in Greece.",
        "Greek"
    );
    olympus.set_location_type(LocationType::Mountain);
    olympus.add_characteristic("Peak reaches into the heavens");
    olympus.add_characteristic("Eternal spring");
    olympus.add_characteristic("Divine palaces of the gods");
    olympus.add_accessibility("Only accessible to gods and invited mortals");
    
    // Underworld (Hades)
    let mut underworld = Location::new(
        "Underworld",
        "Realm of the dead ruled by Hades, where souls go after death.",
        "Greek"
    );
    underworld.set_location_type(LocationType::Underworld);
    underworld.add_characteristic("River Styx");
    underworld.add_characteristic("Fields of Asphodel");
    underworld.add_characteristic("Elysian Fields");
    underworld.add_characteristic("Tartarus");
    underworld.add_accessibility("Entrance near Cumae in Italy");
    underworld.add_accessibility("Achievable through death");
    underworld.add_accessibility("Rare heroes can enter and return alive");
    
    // Delphi
    let mut delphi = Location::new(
        "Delphi",
        "Sacred site with the Temple of Apollo, home to the Oracle of Delphi.",
        "Greek"
    );
    delphi.set_location_type(LocationType::Temple);
    delphi.add_characteristic("Considered the center of the world");
    delphi.add_characteristic("Oracle delivered prophecies");
    delphi.add_characteristic("Built on slopes of Mount Parnassus");
    delphi.add_accessibility("Physical location in Greece");
    delphi.add_accessibility("Sacred precinct required purification to enter");
    
    // Convert to MythEntity
    let olympus_entity = MythEntity::Location(olympus);
    let underworld_entity = MythEntity::Location(underworld);
    let delphi_entity = MythEntity::Location(delphi);
    
    // Add to ontology
    ontology.add_entity(olympus_entity);
    ontology.add_entity(underworld_entity);
    ontology.add_entity(delphi_entity);
}

/// Add Greek concepts to the ontology
fn add_greek_concepts(ontology: &mut MythOntology) {
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
    
    // Convert to MythEntity
    let fate_entity = MythEntity::Concept(fate);
    let hubris_entity = MythEntity::Concept(hubris);
    let xenia_entity = MythEntity::Concept(xenia);
    
    // Add to ontology
    ontology.add_entity(fate_entity);
    ontology.add_entity(hubris_entity);
    ontology.add_entity(xenia_entity);
}