//! Mythological artifacts from various cultures

use crate::core::{MythOntology, MythEntity, Source, SourceType};
use crate::entities::{Artifact, ArtifactType};

/// Create an ontology focused on mythological artifacts from various cultures
pub fn create_artifacts_ontology() -> MythOntology {
    let mut ontology = MythOntology::new();
    
    // Add Greek artifacts
    add_greek_artifacts(&mut ontology);
    
    // Add Norse artifacts
    add_norse_artifacts(&mut ontology);
    
    // Add Celtic artifacts
    add_celtic_artifacts(&mut ontology);
    
    // Add Arthurian artifacts
    add_arthurian_artifacts(&mut ontology);
    
    // Add Middle-Eastern artifacts
    add_middle_eastern_artifacts(&mut ontology);
    
    // Add East Asian artifacts
    add_east_asian_artifacts(&mut ontology);
    
    ontology
}

/// Add Greek mythological artifacts
fn add_greek_artifacts(ontology: &mut MythOntology) {
    let greek_source = Source {
        title: "Greek Mythology Compendium".to_string(),
        author: None,
        year: None,
        source_type: SourceType::CompilationText,
        url: None,
        notes: Some("Collection of Greek mythological texts".to_string()),
    };
    
    // Zeus's Thunderbolt
    let mut thunderbolt = Artifact::new(
        "Zeus's Thunderbolt",
        "The primary weapon of Zeus, capable of creating lightning and thunder.",
        "Greek"
    );
    thunderbolt.set_artifact_type(ArtifactType::Weapon);
    thunderbolt.add_power("Creates lightning");
    thunderbolt.add_power("Produces thunder");
    thunderbolt.add_power("Symbol of Zeus's authority");
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
    aegis.set_owner("Zeus/Athena");
    
    // Hermes's Winged Sandals
    let mut winged_sandals = Artifact::new(
        "Winged Sandals",
        "Magical sandals that allow the wearer to fly as swiftly as any bird.",
        "Greek"
    );
    winged_sandals.set_artifact_type(ArtifactType::Clothing);
    winged_sandals.add_power("Flight");
    winged_sandals.add_power("Super speed");
    winged_sandals.set_creator("Hephaestus");
    winged_sandals.set_owner("Hermes");
    
    // Add to ontology with source
    let mut thunderbolt_entity = MythEntity::Artifact(thunderbolt);
    let mut aegis_entity = MythEntity::Artifact(aegis);
    let mut winged_sandals_entity = MythEntity::Artifact(winged_sandals);
    
    thunderbolt_entity.metadata_mut().add_source(greek_source.clone());
    aegis_entity.metadata_mut().add_source(greek_source.clone());
    winged_sandals_entity.metadata_mut().add_source(greek_source);
    
    ontology.add_entity(thunderbolt_entity);
    ontology.add_entity(aegis_entity);
    ontology.add_entity(winged_sandals_entity);
}

/// Add Norse mythological artifacts
fn add_norse_artifacts(ontology: &mut MythOntology) {
    let norse_source = Source {
        title: "Prose Edda".to_string(),
        author: Some("Snorri Sturluson".to_string()),
        year: Some(1220),
        source_type: SourceType::PrimaryText,
        url: None,
        notes: Some("Old Norse work of literature written in Iceland".to_string()),
    };
    
    // Mjölnir
    let mut mjolnir = Artifact::new(
        "Mjölnir",
        "Thor's hammer, capable of creating lightning and returning to his hand when thrown.",
        "Norse"
    );
    mjolnir.set_artifact_type(ArtifactType::Weapon);
    mjolnir.add_power("Creates lightning and thunder");
    mjolnir.add_power("Returns to wielder when thrown");
    mjolnir.add_power("Only the worthy can lift it");
    mjolnir.set_creator("Dwarves Brokkr and Sindri");
    mjolnir.set_owner("Thor");
    
    // Gungnir
    let mut gungnir = Artifact::new(
        "Gungnir",
        "Odin's spear that never misses its target.",
        "Norse"
    );
    gungnir.set_artifact_type(ArtifactType::Weapon);
    gungnir.add_power("Never misses its target");
    gungnir.add_power("Inscribed with runes");
    gungnir.set_creator("Dwarves");
    gungnir.set_owner("Odin");
    
    // Gleipnir
    let mut gleipnir = Artifact::new(
        "Gleipnir",
        "Magical binding that restrains the monstrous wolf Fenrir.",
        "Norse"
    );
    gleipnir.set_artifact_type(ArtifactType::Other("Binding".to_string()));
    gleipnir.add_power("Unbreakable strength");
    gleipnir.add_power("Made from impossible materials");
    gleipnir.set_creator("Dwarves");
    
    // Add to ontology with source
    let mut mjolnir_entity = MythEntity::Artifact(mjolnir);
    let mut gungnir_entity = MythEntity::Artifact(gungnir);
    let mut gleipnir_entity = MythEntity::Artifact(gleipnir);
    
    mjolnir_entity.metadata_mut().add_source(norse_source.clone());
    gungnir_entity.metadata_mut().add_source(norse_source.clone());
    gleipnir_entity.metadata_mut().add_source(norse_source);
    
    ontology.add_entity(mjolnir_entity);
    ontology.add_entity(gungnir_entity);
    ontology.add_entity(gleipnir_entity);
}

/// Add Celtic mythological artifacts
fn add_celtic_artifacts(ontology: &mut MythOntology) {
    let celtic_source = Source {
        title: "Celtic Mythology Collection".to_string(),
        author: None,
        year: None,
        source_type: SourceType::CompilationText,
        url: None,
        notes: Some("Collection of Celtic mythological tales".to_string()),
    };
    
    // Cauldron of Dagda
    let mut cauldron = Artifact::new(
        "Cauldron of Dagda",
        "Magical cauldron owned by the Dagda that provided unlimited food.",
        "Celtic"
    );
    cauldron.set_artifact_type(ArtifactType::Vessel);
    cauldron.add_power("Provides unlimited food");
    cauldron.add_power("Satisfies everyone according to their merit");
    cauldron.set_owner("Dagda");
    
    // Claíomh Solais
    let mut sword_of_light = Artifact::new(
        "Claíomh Solais",
        "The Sword of Light, an unstoppable weapon in Irish mythology.",
        "Celtic"
    );
    sword_of_light.set_artifact_type(ArtifactType::Weapon);
    sword_of_light.add_power("Emits light");
    sword_of_light.add_power("Unstoppable in battle");
    
    // Add to ontology with source
    let mut cauldron_entity = MythEntity::Artifact(cauldron);
    let mut sword_entity = MythEntity::Artifact(sword_of_light);
    
    cauldron_entity.metadata_mut().add_source(celtic_source.clone());
    sword_entity.metadata_mut().add_source(celtic_source);
    
    ontology.add_entity(cauldron_entity);
    ontology.add_entity(sword_entity);
}

/// Add Arthurian mythological artifacts
fn add_arthurian_artifacts(ontology: &mut MythOntology) {
    let arthurian_source = Source {
        title: "Le Morte d'Arthur".to_string(),
        author: Some("Sir Thomas Malory".to_string()),
        year: Some(1485),
        source_type: SourceType::LiteraryText,
        url: None,
        notes: Some("Compilation of Arthurian tales".to_string()),
    };
    
    // Excalibur
    let mut excalibur = Artifact::new(
        "Excalibur",
        "The legendary sword of King Arthur, given to him by the Lady of the Lake.",
        "Arthurian"
    );
    excalibur.set_artifact_type(ArtifactType::Weapon);
    excalibur.add_power("Unbreakable blade");
    excalibur.add_power("Magically sharp");
    excalibur.add_power("Scabbard prevents bearer from bleeding");
    excalibur.set_creator("Lady of the Lake");
    excalibur.set_owner("King Arthur");
    
    // Holy Grail
    let mut holy_grail = Artifact::new(
        "Holy Grail",
        "The cup used by Jesus at the Last Supper, sought by Arthurian knights.",
        "Arthurian"
    );
    holy_grail.set_artifact_type(ArtifactType::Vessel);
    holy_grail.add_power("Healing");
    holy_grail.add_power("Eternal youth");
    holy_grail.add_power("Spiritual enlightenment");
    
    // Add to ontology with source
    let mut excalibur_entity = MythEntity::Artifact(excalibur);
    let mut grail_entity = MythEntity::Artifact(holy_grail);
    
    excalibur_entity.metadata_mut().add_source(arthurian_source.clone());
    grail_entity.metadata_mut().add_source(arthurian_source);
    
    ontology.add_entity(excalibur_entity);
    ontology.add_entity(grail_entity);
}

/// Add Middle-Eastern mythological artifacts
fn add_middle_eastern_artifacts(ontology: &mut MythOntology) {
    let arabian_source = Source {
        title: "One Thousand and One Nights".to_string(),
        author: None,
        year: Some(1200),
        source_type: SourceType::CompilationText,
        url: None,
        notes: Some("Collection of Middle Eastern folk tales".to_string()),
    };
    
    // Lamp of Aladdin
    let mut aladdin_lamp = Artifact::new(
        "Lamp of Aladdin",
        "Magical oil lamp that contained a djinn who would grant wishes.",
        "Arabian"
    );
    aladdin_lamp.set_artifact_type(ArtifactType::Vessel);
    aladdin_lamp.add_power("Contains a wish-granting djinn");
    aladdin_lamp.add_power("Summoning the djinn when rubbed");
    
    // Solomon's Ring
    let mut solomons_ring = Artifact::new(
        "Ring of Solomon",
        "Magical ring that gave King Solomon the power to command demons and speak with animals.",
        "Middle Eastern"
    );
    solomons_ring.set_artifact_type(ArtifactType::Jewelry);
    solomons_ring.add_power("Command over demons (jinn)");
    solomons_ring.add_power("Ability to speak with animals");
    solomons_ring.add_power("Control over the elements");
    solomons_ring.set_owner("King Solomon");
    
    // Add to ontology with source
    let mut lamp_entity = MythEntity::Artifact(aladdin_lamp);
    let mut ring_entity = MythEntity::Artifact(solomons_ring);
    
    lamp_entity.metadata_mut().add_source(arabian_source.clone());
    ring_entity.metadata_mut().add_source(arabian_source);
    
    ontology.add_entity(lamp_entity);
    ontology.add_entity(ring_entity);
}

/// Add East Asian mythological artifacts
fn add_east_asian_artifacts(ontology: &mut MythOntology) {
    let asian_source = Source {
        title: "East Asian Mythology Collection".to_string(),
        author: None,
        year: None,
        source_type: SourceType::CompilationText,
        url: None,
        notes: Some("Collection of various East Asian mythological texts".to_string()),
    };
    
    // Ruyi Jingu Bang
    let mut ruyi_jingu_bang = Artifact::new(
        "Ruyi Jingu Bang",
        "The magically-expanding iron rod wielded by Sun Wukong (Monkey King) in 'Journey to the West'.",
        "Chinese"
    );
    ruyi_jingu_bang.set_artifact_type(ArtifactType::Weapon);
    ruyi_jingu_bang.add_power("Changes size according to wielder's will");
    ruyi_jingu_bang.add_power("Immense weight (17,550 pounds)");
    ruyi_jingu_bang.add_power("Indestructible");
    ruyi_jingu_bang.set_owner("Sun Wukong (Monkey King)");
    
    // Imperial Regalia of Japan
    let mut kusanagi = Artifact::new(
        "Kusanagi-no-Tsurugi",
        "One of the three Imperial Regalia of Japan, a legendary sword found inside an eight-headed serpent.",
        "Japanese"
    );
    kusanagi.set_artifact_type(ArtifactType::Weapon);
    kusanagi.add_power("Control over wind");
    kusanagi.add_power("Symbol of imperial authority");
    kusanagi.set_owner("Japanese Emperor (symbolically)");
    
    // Add to ontology with source
    let mut ruyi_entity = MythEntity::Artifact(ruyi_jingu_bang);
    let mut kusanagi_entity = MythEntity::Artifact(kusanagi);
    
    ruyi_entity.metadata_mut().add_source(asian_source.clone());
    kusanagi_entity.metadata_mut().add_source(asian_source);
    
    ontology.add_entity(ruyi_entity);
    ontology.add_entity(kusanagi_entity);
}