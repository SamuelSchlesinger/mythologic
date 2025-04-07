use crate::core::{MythOntology, MythEntity, Source, SourceType};
use crate::entities::{Deity, Hero, Creature, Artifact, Location, Concept};
use crate::entities::{Gender, DeityImportance, HeroOrigin, CreatureType, ArtifactType, LocationType, ConceptType};

/// Add Norse entities to the ontology
pub fn add_norse_entities(ontology: &mut MythOntology) {
    // Add deities
    add_norse_deities(ontology);
    
    // Add heroes
    add_norse_heroes(ontology);
    
    // Add creatures
    add_norse_creatures(ontology);
    
    // Add artifacts
    add_norse_artifacts(ontology);
    
    // Add locations
    add_norse_locations(ontology);
    
    // Add concepts
    add_norse_concepts(ontology);
}

/// Add Norse deities to the ontology
fn add_norse_deities(ontology: &mut MythOntology) {
    // Primary source for Norse deities
    let edda_source = Source {
        title: "Poetic Edda".to_string(),
        author: None,
        year: Some(1200),
        source_type: SourceType::PrimaryText,
        url: None,
        notes: Some("Collection of Old Norse poems from the Icelandic medieval manuscript Codex Regius".to_string()),
    };
    
    let prose_edda_source = Source {
        title: "Prose Edda".to_string(),
        author: Some("Snorri Sturluson".to_string()),
        year: Some(1220),
        source_type: SourceType::PrimaryText,
        url: None,
        notes: Some("Old Norse work of literature written in Iceland in the early 13th century".to_string()),
    };
    
    // Odin
    let mut odin = Deity::new(
        "Odin",
        "Chief of the Aesir, god of wisdom, poetry, death, divination, and magic.",
        "Norse"
    );
    odin.add_domain("Wisdom");
    odin.add_domain("War");
    odin.add_domain("Poetry");
    odin.add_domain("Magic");
    odin.add_domain("Death");
    odin.set_gender(Gender::Male);
    odin.set_importance(DeityImportance::Supreme);
    odin.set_pantheon("Aesir");
    odin.add_alternative_name("Wotan");
    odin.add_alternative_name("Woden");
    
    // Thor
    let mut thor = Deity::new(
        "Thor",
        "God of thunder, lightning, storms, oak trees, strength, hallowing, and fertility.",
        "Norse"
    );
    thor.add_domain("Thunder");
    thor.add_domain("Lightning");
    thor.add_domain("Strength");
    thor.add_domain("Protection");
    thor.set_gender(Gender::Male);
    thor.set_importance(DeityImportance::Major);
    thor.set_pantheon("Aesir");
    thor.add_alternative_name("Donar");
    
    // Freyja
    let mut freyja = Deity::new(
        "Freyja",
        "Goddess of love, beauty, fertility, sex, war, gold, and seiðr (magic).",
        "Norse"
    );
    freyja.add_domain("Love");
    freyja.add_domain("Beauty");
    freyja.add_domain("Fertility");
    freyja.add_domain("Magic");
    freyja.add_domain("War");
    freyja.set_gender(Gender::Female);
    freyja.set_importance(DeityImportance::Major);
    freyja.set_pantheon("Vanir");
    
    // Loki
    let mut loki = Deity::new(
        "Loki",
        "Trickster god, shapeshifter, and deity of mischief, sometimes helpful but often causing problems for gods and humans.",
        "Norse"
    );
    loki.add_domain("Trickery");
    loki.add_domain("Fire");
    loki.add_domain("Chaos");
    loki.add_domain("Transformation");
    loki.set_gender(Gender::Male);
    loki.set_importance(DeityImportance::Major);
    loki.set_pantheon("Jotun");
    
    // Heimdall
    let mut heimdall = Deity::new(
        "Heimdall",
        "Watchman of the gods, guardian of Bifrost, the rainbow bridge connecting Asgard to Midgard.",
        "Norse"
    );
    heimdall.add_domain("Vigilance");
    heimdall.add_domain("Light");
    heimdall.add_domain("Guardian");
    heimdall.set_gender(Gender::Male);
    heimdall.set_importance(DeityImportance::Minor);
    heimdall.set_pantheon("Aesir");
    
    // Convert to MythEntity enum and add source metadata
    let mut odin_entity = MythEntity::Deity(odin);
    let mut thor_entity = MythEntity::Deity(thor);
    let mut freyja_entity = MythEntity::Deity(freyja);
    let mut loki_entity = MythEntity::Deity(loki);
    let mut heimdall_entity = MythEntity::Deity(heimdall);
    
    // Add source to metadata
    odin_entity.metadata_mut().add_source(edda_source.clone());
    odin_entity.metadata_mut().add_source(prose_edda_source.clone());
    thor_entity.metadata_mut().add_source(edda_source.clone());
    thor_entity.metadata_mut().add_source(prose_edda_source.clone());
    freyja_entity.metadata_mut().add_source(edda_source.clone());
    freyja_entity.metadata_mut().add_source(prose_edda_source.clone());
    loki_entity.metadata_mut().add_source(edda_source.clone());
    loki_entity.metadata_mut().add_source(prose_edda_source.clone());
    heimdall_entity.metadata_mut().add_source(edda_source.clone());
    heimdall_entity.metadata_mut().add_source(prose_edda_source.clone());
    
    // Add to ontology
    ontology.add_entity(odin_entity);
    ontology.add_entity(thor_entity);
    ontology.add_entity(freyja_entity);
    ontology.add_entity(loki_entity);
    ontology.add_entity(heimdall_entity);
}

/// Add Norse heroes to the ontology
fn add_norse_heroes(ontology: &mut MythOntology) {
    let sagas_source = Source {
        title: "Icelandic Sagas".to_string(),
        author: None,
        year: Some(1200),
        source_type: SourceType::PrimaryText,
        url: None,
        notes: Some("Norse/Icelandic sagas recording historical events and mythology".to_string()),
    };
    
    // Sigurd/Siegfried
    let mut sigurd = Hero::new(
        "Sigurd",
        "Dragon-slayer hero who killed the dragon Fafnir and acquired its treasure.",
        "Norse"
    );
    sigurd.set_origin(HeroOrigin::Mortal);
    sigurd.add_achievement("Slaying the dragon Fafnir");
    sigurd.add_achievement("Acquiring the cursed treasure of Andvari");
    sigurd.add_achievement("Awakening the Valkyrie Brynhildr");
    
    // Beowulf
    let mut beowulf = Hero::new(
        "Beowulf",
        "Legendary Geatish hero who slew the monster Grendel and its mother.",
        "Norse"
    );
    beowulf.set_origin(HeroOrigin::Mortal);
    beowulf.add_achievement("Defeating the monster Grendel");
    beowulf.add_achievement("Slaying Grendel's mother");
    beowulf.add_achievement("Fighting a dragon in old age");
    
    // Convert to MythEntity and add source
    let mut sigurd_entity = MythEntity::Hero(sigurd);
    let mut beowulf_entity = MythEntity::Hero(beowulf);
    
    // Add source to metadata
    sigurd_entity.metadata_mut().add_source(sagas_source.clone());
    beowulf_entity.metadata_mut().add_source(sagas_source);
    
    // Add to ontology
    ontology.add_entity(sigurd_entity);
    ontology.add_entity(beowulf_entity);
}

/// Add Norse creatures to the ontology
fn add_norse_creatures(ontology: &mut MythOntology) {
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
    
    // Convert to MythEntity
    let jormungandr_entity = MythEntity::Creature(jormungandr);
    let fenrir_entity = MythEntity::Creature(fenrir);
    let draugr_entity = MythEntity::Creature(draugr);
    
    // Add to ontology
    ontology.add_entity(jormungandr_entity);
    ontology.add_entity(fenrir_entity);
    ontology.add_entity(draugr_entity);
}

/// Add Norse artifacts to the ontology
fn add_norse_artifacts(ontology: &mut MythOntology) {
    // Mjölnir
    let mut mjolnir = Artifact::new(
        "Mjölnir",
        "Thor's hammer, capable of creating lightning and returning to his hand when thrown.",
        "Norse"
    );
    mjolnir.set_artifact_type(ArtifactType::Weapon);
    mjolnir.add_power("Creates lightning and thunder");
    mjolnir.add_power("Returns to wielder when thrown");
    mjolnir.add_power("Shrinks to be concealed");
    mjolnir.set_creator("Dwarves");
    mjolnir.set_owner("Thor");
    
    // Gungnir
    let mut gungnir = Artifact::new(
        "Gungnir",
        "Odin's spear that never misses its target.",
        "Norse"
    );
    gungnir.set_artifact_type(ArtifactType::Weapon);
    gungnir.add_power("Never misses its target");
    gungnir.add_power("Sanctifies oaths and contracts");
    gungnir.set_creator("Dwarves");
    gungnir.set_owner("Odin");
    
    // Brisingamen
    let mut brisingamen = Artifact::new(
        "Brisingamen",
        "Freyja's necklace, made by four dwarves, a powerful symbol of fertility and beauty.",
        "Norse"
    );
    brisingamen.set_artifact_type(ArtifactType::Jewelry);
    brisingamen.add_power("Enhances beauty");
    brisingamen.add_power("Symbol of fertility");
    brisingamen.set_creator("Four dwarves");
    brisingamen.set_owner("Freyja");
    
    // Convert to MythEntity
    let mjolnir_entity = MythEntity::Artifact(mjolnir);
    let gungnir_entity = MythEntity::Artifact(gungnir);
    let brisingamen_entity = MythEntity::Artifact(brisingamen);
    
    // Add to ontology
    ontology.add_entity(mjolnir_entity);
    ontology.add_entity(gungnir_entity);
    ontology.add_entity(brisingamen_entity);
}

/// Add Norse locations to the ontology
fn add_norse_locations(ontology: &mut MythOntology) {
    // Asgard
    let mut asgard = Location::new(
        "Asgard",
        "Home of the Aesir gods, one of the Nine Worlds connected by Yggdrasil.",
        "Norse"
    );
    asgard.set_location_type(LocationType::Heaven);
    asgard.add_characteristic("Fortified with high walls");
    asgard.add_characteristic("Contains Valhalla and other divine halls");
    asgard.add_characteristic("Located in the sky");
    asgard.add_accessibility("Via Bifrost, the rainbow bridge");
    
    // Midgard
    let mut midgard = Location::new(
        "Midgard",
        "The world of humans, one of the Nine Worlds connected by Yggdrasil.",
        "Norse"
    );
    midgard.set_location_type(LocationType::Other("Human Realm".to_string()));
    midgard.add_characteristic("Encircled by the world serpent Jormungandr");
    midgard.add_characteristic("Created from the body of the giant Ymir");
    midgard.add_characteristic("Connected to other worlds by Yggdrasil");
    midgard.add_accessibility("Physical realm of mortals");
    
    // Valhalla
    let mut valhalla = Location::new(
        "Valhalla",
        "The majestic hall of the slain in Asgard, ruled by Odin, where half of those who die in combat go.",
        "Norse"
    );
    valhalla.set_location_type(LocationType::Afterlife);
    valhalla.add_characteristic("Roof made of shields");
    valhalla.add_characteristic("Spears for rafters");
    valhalla.add_characteristic("Home to the Einherjar (slain warriors)");
    valhalla.add_characteristic("Preparation ground for Ragnarök");
    valhalla.add_accessibility("Die honorably in battle");
    valhalla.add_accessibility("Be chosen by the Valkyries");
    
    // Convert to MythEntity
    let asgard_entity = MythEntity::Location(asgard);
    let midgard_entity = MythEntity::Location(midgard);
    let valhalla_entity = MythEntity::Location(valhalla);
    
    // Add to ontology
    ontology.add_entity(asgard_entity);
    ontology.add_entity(midgard_entity);
    ontology.add_entity(valhalla_entity);
}

/// Add Norse concepts to the ontology
fn add_norse_concepts(ontology: &mut MythOntology) {
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
    
    // Convert to MythEntity
    let ragnarok_entity = MythEntity::Concept(ragnarok);
    let wyrd_entity = MythEntity::Concept(wyrd);
    
    // Add to ontology
    ontology.add_entity(ragnarok_entity);
    ontology.add_entity(wyrd_entity);
}