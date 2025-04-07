//! Mythological locations from various cultures

use crate::core::{MythOntology, MythEntity, Source, SourceType};
use crate::entities::{Location, LocationType};

/// Create an ontology focused on mythological locations from various cultures
pub fn create_locations_ontology() -> MythOntology {
    let mut ontology = MythOntology::new();
    
    // Add Greek locations
    add_greek_locations(&mut ontology);
    
    // Add Norse locations
    add_norse_locations(&mut ontology);
    
    // Add Celtic locations
    add_celtic_locations(&mut ontology);
    
    // Add Middle Eastern locations
    add_middle_eastern_locations(&mut ontology);
    
    // Add East Asian locations
    add_east_asian_locations(&mut ontology);
    
    // Add Hindu locations
    add_hindu_locations(&mut ontology);
    
    ontology
}

/// Add Greek mythological locations
fn add_greek_locations(ontology: &mut MythOntology) {
    let greek_source = Source {
        title: "Greek Mythological Geography".to_string(),
        author: None,
        year: Some(-700),
        source_type: SourceType::CompilationText,
        url: None,
        notes: Some("Collection of Greek mythological locations".to_string()),
    };
    
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
    
    // Elysian Fields
    let mut elysium = Location::new(
        "Elysian Fields",
        "Paradise within the Underworld reserved for heroes and those blessed by the gods.",
        "Greek"
    );
    elysium.set_location_type(LocationType::Afterlife);
    elysium.add_characteristic("Eternal spring");
    elysium.add_characteristic("Peaceful meadows");
    elysium.add_characteristic("No pain or suffering");
    elysium.add_accessibility("Only for the favored dead");
    elysium.add_accessibility("Located within the Underworld");
    
    // Tartarus
    let mut tartarus = Location::new(
        "Tartarus",
        "The deep abyss used as a dungeon of torment for the wicked and prison for the Titans.",
        "Greek"
    );
    tartarus.set_location_type(LocationType::Underworld);
    tartarus.add_characteristic("Deepest part of the cosmos");
    tartarus.add_characteristic("Surrounded by bronze walls");
    tartarus.add_characteristic("Region of punishment");
    tartarus.add_accessibility("Judged as wicked by the gods");
    
    // Add to ontology with source
    let mut olympus_entity = MythEntity::Location(olympus);
    let mut underworld_entity = MythEntity::Location(underworld);
    let mut elysium_entity = MythEntity::Location(elysium);
    let mut tartarus_entity = MythEntity::Location(tartarus);
    
    olympus_entity.metadata_mut().add_source(greek_source.clone());
    underworld_entity.metadata_mut().add_source(greek_source.clone());
    elysium_entity.metadata_mut().add_source(greek_source.clone());
    tartarus_entity.metadata_mut().add_source(greek_source);
    
    ontology.add_entity(olympus_entity);
    ontology.add_entity(underworld_entity);
    ontology.add_entity(elysium_entity);
    ontology.add_entity(tartarus_entity);
}

/// Add Norse mythological locations
fn add_norse_locations(ontology: &mut MythOntology) {
    let norse_source = Source {
        title: "Norse Mythological Cosmography".to_string(),
        author: None,
        year: Some(1200),
        source_type: SourceType::CompilationText,
        url: None,
        notes: Some("Collection of Norse mythological locations".to_string()),
    };
    
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
    
    // Yggdrasil
    let mut yggdrasil = Location::new(
        "Yggdrasil",
        "The immense ash tree that connects the Nine Worlds of Norse cosmology.",
        "Norse"
    );
    yggdrasil.set_location_type(LocationType::Cosmic);
    yggdrasil.add_characteristic("Contains the Nine Worlds");
    yggdrasil.add_characteristic("Inhabited by various creatures");
    yggdrasil.add_characteristic("Central axis of the cosmos");
    yggdrasil.add_accessibility("Divine or magical means");
    
    // Add to ontology with source
    let mut asgard_entity = MythEntity::Location(asgard);
    let mut midgard_entity = MythEntity::Location(midgard);
    let mut valhalla_entity = MythEntity::Location(valhalla);
    let mut yggdrasil_entity = MythEntity::Location(yggdrasil);
    
    asgard_entity.metadata_mut().add_source(norse_source.clone());
    midgard_entity.metadata_mut().add_source(norse_source.clone());
    valhalla_entity.metadata_mut().add_source(norse_source.clone());
    yggdrasil_entity.metadata_mut().add_source(norse_source);
    
    ontology.add_entity(asgard_entity);
    ontology.add_entity(midgard_entity);
    ontology.add_entity(valhalla_entity);
    ontology.add_entity(yggdrasil_entity);
}

/// Add Celtic mythological locations
fn add_celtic_locations(ontology: &mut MythOntology) {
    let celtic_source = Source {
        title: "Celtic Mythological Geography".to_string(),
        author: None,
        year: Some(800),
        source_type: SourceType::CompilationText,
        url: None,
        notes: Some("Collection of Celtic mythological locations".to_string()),
    };
    
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
    
    // Add to ontology with source
    let mut tir_na_nog_entity = MythEntity::Location(tir_na_nog);
    let mut annwn_entity = MythEntity::Location(annwn);
    
    tir_na_nog_entity.metadata_mut().add_source(celtic_source.clone());
    annwn_entity.metadata_mut().add_source(celtic_source);
    
    ontology.add_entity(tir_na_nog_entity);
    ontology.add_entity(annwn_entity);
}

/// Add Middle Eastern mythological locations
fn add_middle_eastern_locations(ontology: &mut MythOntology) {
    let mesopotamian_source = Source {
        title: "Mesopotamian Mythological Geography".to_string(),
        author: None,
        year: Some(-2000),
        source_type: SourceType::CompilationText,
        url: None,
        notes: Some("Collection of Mesopotamian mythological locations".to_string()),
    };
    
    // Garden of Eden
    let mut eden = Location::new(
        "Garden of Eden",
        "The biblical paradise where the first humans lived before the Fall.",
        "Middle Eastern"
    );
    eden.set_location_type(LocationType::Other("Paradise".to_string()));
    eden.add_characteristic("Four rivers flowing from it");
    eden.add_characteristic("Tree of Life");
    eden.add_characteristic("Tree of Knowledge of Good and Evil");
    eden.add_accessibility("Closed to humans after the Fall");
    eden.add_accessibility("Guarded by cherubim with flaming swords");
    
    // Dilmun
    let mut dilmun = Location::new(
        "Dilmun",
        "A paradise-like land in Sumerian mythology, where the immortals dwelled.",
        "Mesopotamian"
    );
    dilmun.set_location_type(LocationType::Other("Paradise".to_string()));
    dilmun.add_characteristic("Pure, clean, and bright");
    dilmun.add_characteristic("No illness or death");
    dilmun.add_characteristic("Sacred to Enki and Ninhursag");
    dilmun.add_accessibility("Located far to the east");
    
    // Add to ontology with source
    let mut eden_entity = MythEntity::Location(eden);
    let mut dilmun_entity = MythEntity::Location(dilmun);
    
    eden_entity.metadata_mut().add_source(mesopotamian_source.clone());
    dilmun_entity.metadata_mut().add_source(mesopotamian_source);
    
    ontology.add_entity(eden_entity);
    ontology.add_entity(dilmun_entity);
}

/// Add East Asian mythological locations
fn add_east_asian_locations(ontology: &mut MythOntology) {
    let chinese_source = Source {
        title: "Chinese Mythological Geography".to_string(),
        author: None,
        year: Some(300),
        source_type: SourceType::CompilationText,
        url: None,
        notes: Some("Collection of Chinese mythological locations".to_string()),
    };
    
    // Mount Kunlun
    let mut kunlun = Location::new(
        "Mount Kunlun",
        "Mythical mountain in Chinese mythology, dwelling place of immortals and gods.",
        "Chinese"
    );
    kunlun.set_location_type(LocationType::Mountain);
    kunlun.add_characteristic("Jade palaces");
    kunlun.add_characteristic("Garden of immortality");
    kunlun.add_characteristic("Home to Xiwangmu (Queen Mother of the West)");
    kunlun.add_accessibility("Difficult journey through mountains");
    kunlun.add_accessibility("Attaining immortality");
    
    // Diyu
    let mut diyu = Location::new(
        "Diyu",
        "The Chinese realm of the dead, an underground maze with various levels of hell.",
        "Chinese"
    );
    diyu.set_location_type(LocationType::Underworld);
    diyu.add_characteristic("Multiple levels of punishment");
    diyu.add_characteristic("Ruled by Yanluo Wang and other judges");
    diyu.add_characteristic("Souls can be reincarnated after serving their time");
    diyu.add_accessibility("After death and judgment");
    
    // Add to ontology with source
    let mut kunlun_entity = MythEntity::Location(kunlun);
    let mut diyu_entity = MythEntity::Location(diyu);
    
    kunlun_entity.metadata_mut().add_source(chinese_source.clone());
    diyu_entity.metadata_mut().add_source(chinese_source);
    
    ontology.add_entity(kunlun_entity);
    ontology.add_entity(diyu_entity);
}

/// Add Hindu mythological locations
fn add_hindu_locations(ontology: &mut MythOntology) {
    let hindu_source = Source {
        title: "Hindu Mythological Cosmography".to_string(),
        author: None,
        year: Some(-500),
        source_type: SourceType::CompilationText,
        url: None,
        notes: Some("Collection of Hindu mythological locations".to_string()),
    };
    
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
    
    // Add to ontology with source
    let mut meru_entity = MythEntity::Location(meru);
    let mut svarga_entity = MythEntity::Location(svarga);
    let mut naraka_entity = MythEntity::Location(naraka);
    
    meru_entity.metadata_mut().add_source(hindu_source.clone());
    svarga_entity.metadata_mut().add_source(hindu_source.clone());
    naraka_entity.metadata_mut().add_source(hindu_source);
    
    ontology.add_entity(meru_entity);
    ontology.add_entity(svarga_entity);
    ontology.add_entity(naraka_entity);
}