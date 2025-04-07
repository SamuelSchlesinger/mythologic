//! Mythological creatures from various cultures

use crate::core::{MythOntology, MythEntity, Source, SourceType};
use crate::entities::{Creature, CreatureType};

/// Create an ontology focused on mythological creatures from various cultures
pub fn create_creatures_ontology() -> MythOntology {
    let mut ontology = MythOntology::new();
    
    // Add Greek creatures
    add_greek_creatures(&mut ontology);
    
    // Add Norse creatures
    add_norse_creatures(&mut ontology);
    
    // Add Celtic creatures
    add_celtic_creatures(&mut ontology);
    
    // Add Egyptian creatures
    add_egyptian_creatures(&mut ontology);
    
    // Add East Asian creatures
    add_east_asian_creatures(&mut ontology);
    
    // Add Slavic creatures
    add_slavic_creatures(&mut ontology);
    
    ontology
}

/// Add Greek mythological creatures
fn add_greek_creatures(ontology: &mut MythOntology) {
    let greek_source = Source {
        title: "Greek Mythological Bestiary".to_string(),
        author: None,
        year: Some(-700),
        source_type: SourceType::CompilationText,
        url: None,
        notes: Some("Collection of Greek mythological creatures".to_string()),
    };
    
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
    
    // Chimera
    let mut chimera = Creature::new(
        "Chimera",
        "Fire-breathing hybrid monster with parts from multiple animals: lion's head, goat's body, and serpent's tail.",
        "Greek"
    );
    chimera.set_creature_type(CreatureType::Hybrid);
    chimera.add_habitat("Lycia");
    chimera.add_ability("Fire breathing");
    chimera.add_ability("Multiple animal parts");
    
    // Siren
    let mut siren = Creature::new(
        "Siren",
        "Dangerous creatures with enchanting voices that lured sailors to shipwreck on rocky coasts.",
        "Greek"
    );
    siren.set_creature_type(CreatureType::Hybrid);
    siren.add_habitat("Islands");
    siren.add_habitat("Rocky coasts");
    siren.add_ability("Enchanting song");
    siren.add_ability("Luring sailors to their doom");
    
    // Add to ontology with source
    let mut minotaur_entity = MythEntity::Creature(minotaur);
    let mut cerberus_entity = MythEntity::Creature(cerberus);
    let mut medusa_entity = MythEntity::Creature(medusa);
    let mut chimera_entity = MythEntity::Creature(chimera);
    let mut siren_entity = MythEntity::Creature(siren);
    
    minotaur_entity.metadata_mut().add_source(greek_source.clone());
    cerberus_entity.metadata_mut().add_source(greek_source.clone());
    medusa_entity.metadata_mut().add_source(greek_source.clone());
    chimera_entity.metadata_mut().add_source(greek_source.clone());
    siren_entity.metadata_mut().add_source(greek_source);
    
    ontology.add_entity(minotaur_entity);
    ontology.add_entity(cerberus_entity);
    ontology.add_entity(medusa_entity);
    ontology.add_entity(chimera_entity);
    ontology.add_entity(siren_entity);
}

/// Add Norse mythological creatures
fn add_norse_creatures(ontology: &mut MythOntology) {
    let norse_source = Source {
        title: "Norse Mythological Bestiary".to_string(),
        author: None,
        year: Some(1200),
        source_type: SourceType::CompilationText,
        url: None,
        notes: Some("Collection of Norse mythological creatures".to_string()),
    };
    
    // Jormungandr (World Serpent)
    let mut jormungandr = Creature::new(
        "Jormungandr",
        "The Midgard Serpent, a massive snake encircling the world and biting its own tail.",
        "Norse"
    );
    jormungandr.set_creature_type(CreatureType::Monster);
    jormungandr.add_habitat("Ocean");
    jormungandr.add_habitat("Midgard");
    jormungandr.add_ability("Enormous size");
    jormungandr.add_ability("Venomous");
    
    // Fenrir
    let mut fenrir = Creature::new(
        "Fenrir",
        "Monstrous wolf destined to kill Odin during Ragnarök.",
        "Norse"
    );
    fenrir.set_creature_type(CreatureType::Monster);
    fenrir.add_habitat("Asgard (bound)");
    fenrir.add_ability("Enormous strength");
    fenrir.add_ability("Able to break any chain except Gleipnir");
    
    // Draugr
    let mut draugr = Creature::new(
        "Draugr",
        "Undead being from Norse mythology with superhuman strength and various magical abilities.",
        "Norse"
    );
    draugr.set_creature_type(CreatureType::Undead);
    draugr.add_habitat("Burial mounds");
    draugr.add_habitat("Graves");
    draugr.add_ability("Superhuman strength");
    draugr.add_ability("Shape-shifting");
    draugr.add_ability("Control over weather");
    
    // Add to ontology with source
    let mut jormungandr_entity = MythEntity::Creature(jormungandr);
    let mut fenrir_entity = MythEntity::Creature(fenrir);
    let mut draugr_entity = MythEntity::Creature(draugr);
    
    jormungandr_entity.metadata_mut().add_source(norse_source.clone());
    fenrir_entity.metadata_mut().add_source(norse_source.clone());
    draugr_entity.metadata_mut().add_source(norse_source);
    
    ontology.add_entity(jormungandr_entity);
    ontology.add_entity(fenrir_entity);
    ontology.add_entity(draugr_entity);
}

/// Add Celtic mythological creatures
fn add_celtic_creatures(ontology: &mut MythOntology) {
    let celtic_source = Source {
        title: "Celtic Mythological Bestiary".to_string(),
        author: None,
        year: Some(800),
        source_type: SourceType::CompilationText,
        url: None,
        notes: Some("Collection of Celtic mythological creatures".to_string()),
    };
    
    // Selkie
    let mut selkie = Creature::new(
        "Selkie",
        "Shapeshifting creature that can transform from seal to human form by shedding its skin.",
        "Celtic"
    );
    selkie.set_creature_type(CreatureType::Shapeshifter);
    selkie.add_habitat("Coastal regions");
    selkie.add_habitat("Ocean");
    selkie.add_ability("Transformation between seal and human form");
    
    // Each Uisge
    let mut each_uisge = Creature::new(
        "Each Uisge",
        "Dangerous water horse that would entice people to ride it, then drown them in lochs or the sea.",
        "Celtic"
    );
    each_uisge.set_creature_type(CreatureType::Shapeshifter);
    each_uisge.add_habitat("Lochs");
    each_uisge.add_habitat("Sea");
    each_uisge.add_ability("Shapeshifting");
    each_uisge.add_ability("Supernatural strength");
    each_uisge.add_ability("Drowning victims");
    
    // Banshee
    let mut banshee = Creature::new(
        "Banshee",
        "Female spirit whose wailing warns of an impending death in a family.",
        "Celtic"
    );
    banshee.set_creature_type(CreatureType::Spirit);
    banshee.add_habitat("Near family homes");
    banshee.add_ability("Death warning");
    banshee.add_ability("Keening (wailing)");
    banshee.add_ability("Foretelling death");
    
    // Add to ontology with source
    let mut selkie_entity = MythEntity::Creature(selkie);
    let mut each_uisge_entity = MythEntity::Creature(each_uisge);
    let mut banshee_entity = MythEntity::Creature(banshee);
    
    selkie_entity.metadata_mut().add_source(celtic_source.clone());
    each_uisge_entity.metadata_mut().add_source(celtic_source.clone());
    banshee_entity.metadata_mut().add_source(celtic_source);
    
    ontology.add_entity(selkie_entity);
    ontology.add_entity(each_uisge_entity);
    ontology.add_entity(banshee_entity);
}

/// Add Egyptian mythological creatures
fn add_egyptian_creatures(ontology: &mut MythOntology) {
    let egyptian_source = Source {
        title: "Egyptian Mythological Bestiary".to_string(),
        author: None,
        year: Some(-2000),
        source_type: SourceType::CompilationText,
        url: None,
        notes: Some("Collection of Egyptian mythological creatures".to_string()),
    };
    
    // Sphinx
    let mut sphinx = Creature::new(
        "Sphinx",
        "Creature with the body of a lion and the head of a human, known for posing riddles.",
        "Egyptian"
    );
    sphinx.set_creature_type(CreatureType::Hybrid);
    sphinx.add_habitat("Desert");
    sphinx.add_ability("Intelligence");
    sphinx.add_ability("Posing riddles");
    sphinx.add_ability("Guardian of sacred sites");
    
    // Ammit
    let mut ammit = Creature::new(
        "Ammit",
        "Devourer of the dead, with the head of a crocodile, forequarters of a lion, and hindquarters of a hippopotamus.",
        "Egyptian"
    );
    ammit.set_creature_type(CreatureType::Hybrid);
    ammit.add_habitat("Duat (Underworld)");
    ammit.add_ability("Devouring hearts of the unworthy dead");
    ammit.add_ability("Destroying souls");
    
    // Add to ontology with source
    let mut sphinx_entity = MythEntity::Creature(sphinx);
    let mut ammit_entity = MythEntity::Creature(ammit);
    
    sphinx_entity.metadata_mut().add_source(egyptian_source.clone());
    ammit_entity.metadata_mut().add_source(egyptian_source);
    
    ontology.add_entity(sphinx_entity);
    ontology.add_entity(ammit_entity);
}

/// Add East Asian mythological creatures
fn add_east_asian_creatures(ontology: &mut MythOntology) {
    let asian_source = Source {
        title: "East Asian Mythological Bestiary".to_string(),
        author: None,
        year: None,
        source_type: SourceType::CompilationText,
        url: None,
        notes: Some("Collection of East Asian mythological creatures".to_string()),
    };
    
    // Long (Chinese Dragon)
    let mut long = Creature::new(
        "Long",
        "Chinese dragon, divine water deity associated with rainfall, floods, and bodies of water.",
        "Chinese"
    );
    long.set_creature_type(CreatureType::Dragon);
    long.add_habitat("Water");
    long.add_habitat("Sky");
    long.add_ability("Weather control");
    long.add_ability("Flight");
    long.add_ability("Divine wisdom");
    
    // Kitsune
    let mut kitsune = Creature::new(
        "Kitsune",
        "Intelligent fox spirit that can shapeshift into human form, growing additional tails as it ages.",
        "Japanese"
    );
    kitsune.set_creature_type(CreatureType::Shapeshifter);
    kitsune.add_habitat("Forests");
    kitsune.add_habitat("Mountains");
    kitsune.add_ability("Shapeshifting");
    kitsune.add_ability("Illusion creation");
    kitsune.add_ability("Possession");
    
    // Jiangshi
    let mut jiangshi = Creature::new(
        "Jiangshi",
        "Reanimated corpse that hops around absorbing life energy from the living.",
        "Chinese"
    );
    jiangshi.set_creature_type(CreatureType::Undead);
    jiangshi.add_habitat("Graveyards");
    jiangshi.add_ability("Hopping movement");
    jiangshi.add_ability("Absorbing qi (life energy)");
    jiangshi.add_ability("Superhuman strength");
    
    // Add to ontology with source
    let mut long_entity = MythEntity::Creature(long);
    let mut kitsune_entity = MythEntity::Creature(kitsune);
    let mut jiangshi_entity = MythEntity::Creature(jiangshi);
    
    long_entity.metadata_mut().add_source(asian_source.clone());
    kitsune_entity.metadata_mut().add_source(asian_source.clone());
    jiangshi_entity.metadata_mut().add_source(asian_source);
    
    ontology.add_entity(long_entity);
    ontology.add_entity(kitsune_entity);
    ontology.add_entity(jiangshi_entity);
}

/// Add Slavic mythological creatures
fn add_slavic_creatures(ontology: &mut MythOntology) {
    let slavic_source = Source {
        title: "Slavic Mythological Bestiary".to_string(),
        author: None,
        year: Some(1000),
        source_type: SourceType::CompilationText,
        url: None,
        notes: Some("Collection of Slavic mythological creatures".to_string()),
    };
    
    // Baba Yaga
    let mut baba_yaga = Creature::new(
        "Baba Yaga",
        "Supernatural being who appears as a deformed or ferocious-looking old woman, living in a hut on chicken legs.",
        "Slavic"
    );
    baba_yaga.set_creature_type(CreatureType::Other("Witch".to_string()));
    baba_yaga.add_habitat("Forests");
    baba_yaga.add_ability("Flying in a mortar");
    baba_yaga.add_ability("Magic");
    baba_yaga.add_ability("Knowledge of secrets");
    
    // Vodyanoy
    let mut vodyanoy = Creature::new(
        "Vodyanoy",
        "Water spirit who would drown people who swam in his waters.",
        "Slavic"
    );
    vodyanoy.set_creature_type(CreatureType::Spirit);
    vodyanoy.add_habitat("Lakes");
    vodyanoy.add_habitat("Rivers");
    vodyanoy.add_ability("Drowning victims");
    vodyanoy.add_ability("Control over water");
    vodyanoy.add_ability("Shapeshifting");
    
    // Firebird
    let mut firebird = Creature::new(
        "Firebird",
        "Magical glowing bird from Slavic folklore that is both a blessing and a bringer of doom to its captor.",
        "Slavic"
    );
    firebird.set_creature_type(CreatureType::Other("Magical Beast".to_string()));
    firebird.add_habitat("Distant lands");
    firebird.add_ability("Glowing feathers");
    firebird.add_ability("Flight");
    firebird.add_ability("Bringing both fortune and misfortune");
    
    // Add to ontology with source
    let mut baba_yaga_entity = MythEntity::Creature(baba_yaga);
    let mut vodyanoy_entity = MythEntity::Creature(vodyanoy);
    let mut firebird_entity = MythEntity::Creature(firebird);
    
    baba_yaga_entity.metadata_mut().add_source(slavic_source.clone());
    vodyanoy_entity.metadata_mut().add_source(slavic_source.clone());
    firebird_entity.metadata_mut().add_source(slavic_source);
    
    ontology.add_entity(baba_yaga_entity);
    ontology.add_entity(vodyanoy_entity);
    ontology.add_entity(firebird_entity);
}