//! Celtic mythology example ontology

use crate::core::{MythOntology, MythEntity, Source, SourceType};
use crate::entities::{Deity, Hero, Artifact, Location};
use crate::entities::{Gender, DeityImportance, HeroOrigin, ArtifactType, LocationType};
use crate::cultural::{Pantheon, Culture, TimePeriod};

/// Create a comprehensive Celtic mythology ontology
pub fn create_celtic_ontology() -> MythOntology {
    let mut ontology = MythOntology::new();
    
    // Add Celtic deities
    add_celtic_deities(&mut ontology);
    
    // Add Celtic heroes
    add_celtic_heroes(&mut ontology);
    
    // Add Celtic locations
    add_celtic_locations(&mut ontology);
    
    // Add Celtic artifacts
    add_celtic_artifacts(&mut ontology);
    
    // Add Celtic culture
    add_celtic_culture(&mut ontology);
    
    // Add Celtic pantheon
    add_celtic_pantheon(&mut ontology);
    
    ontology
}

/// Add Celtic deities to the ontology
fn add_celtic_deities(ontology: &mut MythOntology) {
    // Primary source for Celtic deities
    let irish_source = Source {
        title: "Lebor Gabála Érenn".to_string(),
        author: None,
        year: Some(1100),
        source_type: SourceType::CompilationText,
        url: None,
        notes: Some("Book of the Taking of Ireland, a collection of poems and prose narratives".to_string()),
    };
    
    // The Dagda
    let mut dagda = Deity::new(
        "The Dagda",
        "Father figure and chief of the Tuatha Dé Danann, associated with fertility, agriculture, magic, and wisdom.",
        "Celtic"
    );
    dagda.add_domain("Fertility");
    dagda.add_domain("Agriculture");
    dagda.add_domain("Magic");
    dagda.add_domain("Wisdom");
    dagda.set_gender(Gender::Male);
    dagda.set_importance(DeityImportance::Supreme);
    dagda.set_pantheon("Tuatha Dé Danann");
    dagda.add_alternative_name("Eochaid Ollathair");
    
    // Lugh
    let mut lugh = Deity::new(
        "Lugh",
        "God of skill, crafts, and light, master of all arts, associated with the harvest festival Lughnasadh.",
        "Celtic"
    );
    lugh.add_domain("Light");
    lugh.add_domain("Skill");
    lugh.add_domain("Arts");
    lugh.add_domain("Crafts");
    lugh.set_gender(Gender::Male);
    lugh.set_importance(DeityImportance::Major);
    lugh.set_pantheon("Tuatha Dé Danann");
    lugh.add_alternative_name("Lugh Lámhfhada");
    
    // Morrígan
    let mut morrigan = Deity::new(
        "The Morrígan",
        "Goddess of war, fate, and death, often appearing as a trio of goddesses or in different forms.",
        "Celtic"
    );
    morrigan.add_domain("War");
    morrigan.add_domain("Fate");
    morrigan.add_domain("Death");
    morrigan.add_domain("Sovereignty");
    morrigan.set_gender(Gender::Female);
    morrigan.set_importance(DeityImportance::Major);
    morrigan.set_pantheon("Tuatha Dé Danann");
    morrigan.add_alternative_name("Phantom Queen");
    
    // Brigid
    let mut brigid = Deity::new(
        "Brigid",
        "Goddess of poetry, healing, and smithcraft, associated with fire and springtime.",
        "Celtic"
    );
    brigid.add_domain("Poetry");
    brigid.add_domain("Healing");
    brigid.add_domain("Fire");
    brigid.add_domain("Smithcraft");
    brigid.set_gender(Gender::Female);
    brigid.set_importance(DeityImportance::Major);
    brigid.set_pantheon("Tuatha Dé Danann");
    brigid.add_alternative_name("Brighid");
    
    // Cernunnos
    let mut cernunnos = Deity::new(
        "Cernunnos",
        "Horned god of animals, fertility, and the underworld.",
        "Celtic"
    );
    cernunnos.add_domain("Animals");
    cernunnos.add_domain("Fertility");
    cernunnos.add_domain("Underworld");
    cernunnos.add_domain("Wilderness");
    cernunnos.set_gender(Gender::Male);
    cernunnos.set_importance(DeityImportance::Major);
    cernunnos.set_pantheon("Gaulish");
    
    // Convert to MythEntity enum and add source
    let mut dagda_entity = MythEntity::Deity(dagda);
    let mut lugh_entity = MythEntity::Deity(lugh);
    let mut morrigan_entity = MythEntity::Deity(morrigan);
    let mut brigid_entity = MythEntity::Deity(brigid);
    let mut cernunnos_entity = MythEntity::Deity(cernunnos);
    
    // Add source to metadata
    dagda_entity.metadata_mut().add_source(irish_source.clone());
    lugh_entity.metadata_mut().add_source(irish_source.clone());
    morrigan_entity.metadata_mut().add_source(irish_source.clone());
    brigid_entity.metadata_mut().add_source(irish_source.clone());
    cernunnos_entity.metadata_mut().add_source(irish_source);
    
    // Add to ontology
    ontology.add_entity(dagda_entity);
    ontology.add_entity(lugh_entity);
    ontology.add_entity(morrigan_entity);
    ontology.add_entity(brigid_entity);
    ontology.add_entity(cernunnos_entity);
}

/// Add Celtic heroes to the ontology
fn add_celtic_heroes(ontology: &mut MythOntology) {
    let celtic_source = Source {
        title: "Celtic Heroic Cycles".to_string(),
        author: None,
        year: Some(800),
        source_type: SourceType::CompilationText,
        url: None,
        notes: Some("Collection of Irish and Welsh mythological tales".to_string()),
    };
    
    // Cú Chulainn
    let mut cu_chulainn = Hero::new(
        "Cú Chulainn",
        "Irish hero from Ulster who underwent a battle frenzy (ríastrad), famous for his single-handed defense of Ulster.",
        "Celtic"
    );
    cu_chulainn.set_origin(HeroOrigin::Demigod);
    cu_chulainn.add_achievement("Single-handedly defending Ulster in the Táin Bó Cúailnge");
    cu_chulainn.add_achievement("Killing the hound of Culann");
    cu_chulainn.add_achievement("Defeating the warrior Ferdiad");
    
    // Fionn mac Cumhaill
    let mut fionn = Hero::new(
        "Fionn mac Cumhaill",
        "Mythical hunter-warrior of Irish mythology, leader of the Fianna.",
        "Celtic"
    );
    fionn.set_origin(HeroOrigin::BlessedMortal);
    fionn.add_achievement("Leading the Fianna warriors");
    fionn.add_achievement("Gaining wisdom from the Salmon of Knowledge");
    fionn.add_achievement("Creating the Giant's Causeway in battle with a Scottish giant");
    
    // Add to ontology with source
    let mut cu_chulainn_entity = MythEntity::Hero(cu_chulainn);
    let mut fionn_entity = MythEntity::Hero(fionn);
    
    cu_chulainn_entity.metadata_mut().add_source(celtic_source.clone());
    fionn_entity.metadata_mut().add_source(celtic_source);
    
    ontology.add_entity(cu_chulainn_entity);
    ontology.add_entity(fionn_entity);
}

/// Add Celtic locations to the ontology
fn add_celtic_locations(ontology: &mut MythOntology) {
    // Tír na nÓg
    let mut tir_na_nog = Location::new(
        "Tír na nÓg",
        "The Land of Youth, a supernatural realm of everlasting youth, beauty, health, and joy.",
        "Celtic"
    );
    tir_na_nog.set_location_type(LocationType::Afterlife);
    tir_na_nog.add_characteristic("Eternal youth and beauty");
    tir_na_nog.add_characteristic("Abundant food and drink");
    tir_na_nog.add_characteristic("No illness or death");
    tir_na_nog.add_accessibility("Through invitation from one of its inhabitants");
    tir_na_nog.add_accessibility("Via magical transport across the western sea");
    
    // Annwn
    let mut annwn = Location::new(
        "Annwn",
        "The Otherworld in Welsh mythology, ruled by Arawn or Gwyn ap Nudd.",
        "Celtic"
    );
    annwn.set_location_type(LocationType::Underworld);
    annwn.add_characteristic("Realm of delights and eternal youth");
    annwn.add_characteristic("Source of wisdom and inspiration");
    annwn.add_characteristic("Home to supernatural beings");
    annwn.add_accessibility("Through magical means");
    annwn.add_accessibility("Via specific places in the landscape");
    
    // Emain Macha
    let mut emain_macha = Location::new(
        "Emain Macha",
        "Legendary capital of Ulster and home of the Red Branch Knights in the Ulster Cycle.",
        "Celtic"
    );
    emain_macha.set_location_type(LocationType::City);
    emain_macha.add_characteristic("Seat of King Conchobar mac Nessa");
    emain_macha.add_characteristic("Home of the Red Branch Knights");
    emain_macha.add_characteristic("Location of Cú Chulainn's training");
    emain_macha.add_accessibility("Physical location in Ireland");
    
    // Add to ontology
    ontology.add_entity(MythEntity::Location(tir_na_nog));
    ontology.add_entity(MythEntity::Location(annwn));
    ontology.add_entity(MythEntity::Location(emain_macha));
}

/// Add Celtic artifacts to the ontology
fn add_celtic_artifacts(ontology: &mut MythOntology) {
    // Cauldron of Dagda
    let mut cauldron = Artifact::new(
        "Cauldron of Dagda",
        "Magical cauldron owned by the Dagda that provided unlimited food.",
        "Celtic"
    );
    cauldron.set_artifact_type(ArtifactType::Vessel);
    cauldron.add_power("Provides unlimited food");
    cauldron.add_power("Satisfies everyone according to their merit");
    cauldron.set_owner("The Dagda");
    
    // Claíomh Solais
    let mut sword_of_light = Artifact::new(
        "Claíomh Solais",
        "The Sword of Light, an unstoppable weapon in Irish mythology.",
        "Celtic"
    );
    sword_of_light.set_artifact_type(ArtifactType::Weapon);
    sword_of_light.add_power("Emits light");
    sword_of_light.add_power("Unstoppable in battle");
    
    // Gáe Bulg
    let mut gae_bulg = Artifact::new(
        "Gáe Bulg",
        "The spear of Cú Chulainn, made from the bone of a sea monster.",
        "Celtic"
    );
    gae_bulg.set_artifact_type(ArtifactType::Weapon);
    gae_bulg.add_power("Creates thirty barbs when entering the body");
    gae_bulg.add_power("Cannot be removed once it enters the body");
    gae_bulg.set_owner("Cú Chulainn");
    
    // Add to ontology
    ontology.add_entity(MythEntity::Artifact(cauldron));
    ontology.add_entity(MythEntity::Artifact(sword_of_light));
    ontology.add_entity(MythEntity::Artifact(gae_bulg));
}

/// Add Celtic culture to the ontology
fn add_celtic_culture(ontology: &mut MythOntology) {
    // Create Celtic culture
    let mut celtic_culture = Culture::new(
        "Celtic",
        "The culture of the Celtic peoples of Europe, from the Iron Age through medieval times."
    );
    
    // Add regions
    celtic_culture.add_region("Ireland");
    celtic_culture.add_region("Scotland");
    celtic_culture.add_region("Wales");
    celtic_culture.add_region("Brittany");
    celtic_culture.add_region("Gaul");
    
    // Add languages
    celtic_culture.add_language("Old Irish");
    celtic_culture.add_language("Middle Welsh");
    celtic_culture.add_language("Gaulish");
    celtic_culture.add_language("Brythonic");
    
    // Add time periods
    celtic_culture.add_time_period(TimePeriod {
        name: "La Tène Period".to_string(),
        start_year: Some(-450),
        end_year: Some(-50),
        description: Some("Major Celtic cultural period characterized by distinctive art styles".to_string()),
    });
    
    celtic_culture.add_time_period(TimePeriod {
        name: "Early Medieval Celtic Period".to_string(),
        start_year: Some(400),
        end_year: Some(900),
        description: Some("Period of Celtic Christianity and written preservation of oral traditions".to_string()),
    });
    
    // Add cultural practices
    celtic_culture.add_cultural_practice("Druidic rituals");
    celtic_culture.add_cultural_practice("Bardic tradition");
    celtic_culture.add_cultural_practice("Seasonal festivals");
    celtic_culture.add_cultural_practice("Warrior culture");
    
    // Add to ontology
    ontology.add_entity(MythEntity::Culture(celtic_culture));
}

/// Add Celtic pantheon to the ontology
fn add_celtic_pantheon(ontology: &mut MythOntology) {
    // Find entity IDs
    let dagda_id = find_entity_id(ontology, "The Dagda");
    let lugh_id = find_entity_id(ontology, "Lugh");
    let morrigan_id = find_entity_id(ontology, "The Morrígan");
    let brigid_id = find_entity_id(ontology, "Brigid");
    
    // Create Tuatha Dé Danann pantheon
    let mut tuatha_pantheon = Pantheon::new(
        "Tuatha Dé Danann",
        "The main tribe of gods in Irish mythology, descendants of the goddess Danu.",
        "Celtic"
    );
    
    // Add primary deities to the pantheon
    if let Some(dagda_id) = &dagda_id {
        tuatha_pantheon.add_primary_deity(dagda_id.clone());
    }
    
    if let Some(lugh_id) = &lugh_id {
        tuatha_pantheon.add_primary_deity(lugh_id.clone());
    }
    
    if let Some(morrigan_id) = &morrigan_id {
        tuatha_pantheon.add_primary_deity(morrigan_id.clone());
    }
    
    if let Some(brigid_id) = &brigid_id {
        tuatha_pantheon.add_primary_deity(brigid_id.clone());
    }
    
    // Set cosmology
    tuatha_pantheon.set_cosmology(
        "The Celtic cosmology included multiple worlds: the mortal world, the Otherworld (Tír na nÓg, Annwn), \
        and various supernatural realms. These worlds could intersect at certain times and places, such as at \
        sacred locations, during festivals like Samhain, or via fairy mounds and other entrances to the Otherworld. \
        The gods of the Tuatha Dé Danann were believed to inhabit these otherworldly realms."
    );
    
    // Add founding myth
    tuatha_pantheon.set_founding_myth(
        "According to Irish mythology, the Tuatha Dé Danann (People of the goddess Danu) arrived in Ireland \
        on dark clouds, bringing with them four magical treasures: the Dagda's Cauldron, the Spear of Lugh, \
        the Stone of Fal, and the Sword of Light. They defeated the Fir Bolg in the First Battle of Mag Tuired, \
        and later the Fomorians in the Second Battle of Mag Tuired. Eventually, they were defeated by the Milesians \
        (Gaels) and retreated to the Otherworld, becoming the Aos Sí (fairy folk)."
    );
    
    // Add to ontology
    ontology.add_entity(MythEntity::Pantheon(tuatha_pantheon));
}

/// Helper function to find an entity ID by name
fn find_entity_id(ontology: &MythOntology, name: &str) -> Option<crate::core::MythId> {
    ontology.all_entities().iter()
        .find(|entity| entity.name() == name)
        .map(|entity| entity.id().clone())
}