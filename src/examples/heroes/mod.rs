//! Mythological heroes from various cultures

use crate::core::{MythOntology, MythEntity, Source, SourceType};
use crate::entities::{Hero, HeroOrigin};

/// Create an ontology focused on mythological heroes from various cultures
pub fn create_heroes_ontology() -> MythOntology {
    let mut ontology = MythOntology::new();
    
    // Add Greek heroes
    add_greek_heroes(&mut ontology);
    
    // Add Norse heroes
    add_norse_heroes(&mut ontology);
    
    // Add Celtic heroes
    add_celtic_heroes(&mut ontology);
    
    // Add Mesopotamian heroes
    add_mesopotamian_heroes(&mut ontology);
    
    // Add East Asian heroes
    add_east_asian_heroes(&mut ontology);
    
    // Add Hindu heroes
    add_hindu_heroes(&mut ontology);
    
    ontology
}

/// Add Greek mythological heroes
fn add_greek_heroes(ontology: &mut MythOntology) {
    let greek_source = Source {
        title: "Greek Heroic Epics".to_string(),
        author: None,
        year: Some(-700),
        source_type: SourceType::CompilationText,
        url: None,
        notes: Some("Collection of ancient Greek heroic myths".to_string()),
    };
    
    // Heracles (Hercules)
    let mut heracles = Hero::new(
        "Heracles",
        "Greatest of the Greek heroes, known for his extraordinary strength, courage, and completing the twelve labors.",
        "Greek"
    );
    heracles.set_origin(HeroOrigin::Demigod);
    heracles.add_achievement("Twelve Labors of Heracles");
    heracles.add_achievement("Killing the Nemean Lion");
    heracles.add_achievement("Capturing the Erymanthian Boar");
    heracles.add_achievement("Cleaning the Augean Stables");
    heracles.add_achievement("Stealing the Apples of the Hesperides");
    
    // Achilles
    let mut achilles = Hero::new(
        "Achilles",
        "Greek hero of the Trojan War and central character of Homer's Iliad, known for his invulnerability except for his heel.",
        "Greek"
    );
    achilles.set_origin(HeroOrigin::Demigod);
    achilles.add_achievement("Greatest warrior of the Trojan War");
    achilles.add_achievement("Killing Hector, prince of Troy");
    achilles.add_achievement("Leading the Myrmidons");
    
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
    
    // Theseus
    let mut theseus = Hero::new(
        "Theseus",
        "Athenian hero who slew the Minotaur in the Labyrinth of Crete.",
        "Greek"
    );
    theseus.set_origin(HeroOrigin::Demigod);
    theseus.add_achievement("Slaying the Minotaur");
    theseus.add_achievement("Unifying Attica");
    theseus.add_achievement("Defeating various bandits and monsters on his journey to Athens");
    
    // Add to ontology with source
    let mut heracles_entity = MythEntity::Hero(heracles);
    let mut achilles_entity = MythEntity::Hero(achilles);
    let mut odysseus_entity = MythEntity::Hero(odysseus);
    let mut theseus_entity = MythEntity::Hero(theseus);
    
    heracles_entity.metadata_mut().add_source(greek_source.clone());
    achilles_entity.metadata_mut().add_source(greek_source.clone());
    odysseus_entity.metadata_mut().add_source(greek_source.clone());
    theseus_entity.metadata_mut().add_source(greek_source);
    
    ontology.add_entity(heracles_entity);
    ontology.add_entity(achilles_entity);
    ontology.add_entity(odysseus_entity);
    ontology.add_entity(theseus_entity);
}

/// Add Norse mythological heroes
fn add_norse_heroes(ontology: &mut MythOntology) {
    let norse_source = Source {
        title: "Norse Sagas".to_string(),
        author: None,
        year: Some(1200),
        source_type: SourceType::CompilationText,
        url: None,
        notes: Some("Collection of Norse heroic sagas".to_string()),
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
    
    // Ragnar Lothbrok
    let mut ragnar = Hero::new(
        "Ragnar Lothbrok",
        "Legendary Norse hero and king, known for his raids and battles against the English and French.",
        "Norse"
    );
    ragnar.set_origin(HeroOrigin::Mortal);
    ragnar.add_achievement("Slaying a giant serpent as a young man");
    ragnar.add_achievement("Leading successful raids throughout Europe");
    ragnar.add_achievement("Fathering sons who became famous warriors");
    
    // Add to ontology with source
    let mut sigurd_entity = MythEntity::Hero(sigurd);
    let mut beowulf_entity = MythEntity::Hero(beowulf);
    let mut ragnar_entity = MythEntity::Hero(ragnar);
    
    sigurd_entity.metadata_mut().add_source(norse_source.clone());
    beowulf_entity.metadata_mut().add_source(norse_source.clone());
    ragnar_entity.metadata_mut().add_source(norse_source);
    
    ontology.add_entity(sigurd_entity);
    ontology.add_entity(beowulf_entity);
    ontology.add_entity(ragnar_entity);
}

/// Add Celtic mythological heroes
fn add_celtic_heroes(ontology: &mut MythOntology) {
    let celtic_source = Source {
        title: "Celtic Mythological Cycles".to_string(),
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

/// Add Mesopotamian mythological heroes
fn add_mesopotamian_heroes(ontology: &mut MythOntology) {
    let mesopotamian_source = Source {
        title: "Epic of Gilgamesh".to_string(),
        author: None,
        year: Some(-2100),
        source_type: SourceType::PrimaryText,
        url: None,
        notes: Some("Ancient Mesopotamian epic poem".to_string()),
    };
    
    // Gilgamesh
    let mut gilgamesh = Hero::new(
        "Gilgamesh",
        "King of Uruk who embarks on a journey to find the secret of eternal life after the death of his friend Enkidu.",
        "Mesopotamian"
    );
    gilgamesh.set_origin(HeroOrigin::Demigod);
    gilgamesh.add_achievement("Defeating the monster Humbaba");
    gilgamesh.add_achievement("Slaying the Bull of Heaven");
    gilgamesh.add_achievement("Finding Utnapishtim, survivor of the Great Flood");
    gilgamesh.add_achievement("Temporarily obtaining the plant of immortality");
    
    // Enkidu
    let mut enkidu = Hero::new(
        "Enkidu",
        "Wild man created by the gods to balance Gilgamesh, who becomes his closest friend.",
        "Mesopotamian"
    );
    enkidu.set_origin(HeroOrigin::Other("Divine Creation".to_string()));
    enkidu.add_achievement("Wrestling Gilgamesh to a standstill");
    enkidu.add_achievement("Helping Gilgamesh defeat Humbaba");
    enkidu.add_achievement("Helping defeat the Bull of Heaven");
    
    // Add to ontology with source
    let mut gilgamesh_entity = MythEntity::Hero(gilgamesh);
    let mut enkidu_entity = MythEntity::Hero(enkidu);
    
    gilgamesh_entity.metadata_mut().add_source(mesopotamian_source.clone());
    enkidu_entity.metadata_mut().add_source(mesopotamian_source);
    
    ontology.add_entity(gilgamesh_entity);
    ontology.add_entity(enkidu_entity);
}

/// Add East Asian mythological heroes
fn add_east_asian_heroes(ontology: &mut MythOntology) {
    let chinese_source = Source {
        title: "Journey to the West".to_string(),
        author: Some("Wu Cheng'en".to_string()),
        year: Some(1592),
        source_type: SourceType::LiteraryText,
        url: None,
        notes: Some("Chinese novel, one of the Four Great Classical Novels".to_string()),
    };
    
    let japanese_source = Source {
        title: "Japanese Legendary Cycles".to_string(),
        author: None,
        year: None,
        source_type: SourceType::CompilationText,
        url: None,
        notes: Some("Collection of Japanese historical and mythological tales".to_string()),
    };
    
    // Sun Wukong (Monkey King)
    let mut sun_wukong = Hero::new(
        "Sun Wukong",
        "The Monkey King, a divine trickster with immense strength and magical powers who accompanies the monk Xuanzang on a journey to retrieve Buddhist texts.",
        "Chinese"
    );
    sun_wukong.set_origin(HeroOrigin::Transformed);
    sun_wukong.add_achievement("Born from a stone");
    sun_wukong.add_achievement("Mastering the 72 transformations");
    sun_wukong.add_achievement("Rebelling against Heaven");
    sun_wukong.add_achievement("Protecting Xuanzang on the journey to the West");
    
    // Yamato Takeru
    let mut yamato_takeru = Hero::new(
        "Yamato Takeru",
        "Legendary Japanese prince and warrior celebrated for his courage and military prowess.",
        "Japanese"
    );
    yamato_takeru.set_origin(HeroOrigin::BlessedMortal);
    yamato_takeru.add_achievement("Slaying the Kumaso brothers");
    yamato_takeru.add_achievement("Conquering the eastern regions of Japan");
    yamato_takeru.add_achievement("Using the Kusanagi sword to escape a grass fire trap");
    
    // Add to ontology with source
    let mut sun_wukong_entity = MythEntity::Hero(sun_wukong);
    let mut yamato_takeru_entity = MythEntity::Hero(yamato_takeru);
    
    sun_wukong_entity.metadata_mut().add_source(chinese_source);
    yamato_takeru_entity.metadata_mut().add_source(japanese_source);
    
    ontology.add_entity(sun_wukong_entity);
    ontology.add_entity(yamato_takeru_entity);
}

/// Add Hindu mythological heroes
fn add_hindu_heroes(ontology: &mut MythOntology) {
    let hindu_source = Source {
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
    
    arjuna_entity.metadata_mut().add_source(hindu_source.clone());
    rama_entity.metadata_mut().add_source(hindu_source.clone());
    hanuman_entity.metadata_mut().add_source(hindu_source);
    
    ontology.add_entity(arjuna_entity);
    ontology.add_entity(rama_entity);
    ontology.add_entity(hanuman_entity);
}