use crate::core::{MythOntology, Metadata};
use crate::entities::{Deity, Gender, DeityImportance};
use crate::cultural::{Pantheon, Culture, TimePeriod};
use crate::relationships::{FamilyRelationship, FamilyRelationshipType};

/// Create an example Greek pantheon with some deities
pub fn create_greek_example() -> MythOntology {
    let mut ontology = MythOntology::new();
    
    // Create Zeus
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
    zeus.add_alternative_name("Jupiter");
    
    // Create Hera
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
    hera.add_alternative_name("Juno");
    
    // Create Athena
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
    athena.add_alternative_name("Minerva");
    
    // Add source to metadata
    let mut source = crate::core::Source {
        title: "Theogony".to_string(),
        author: Some("Hesiod".to_string()),
        year: Some(-700),
        source_type: crate::core::SourceType::PrimaryText,
        url: None,
        notes: Some("Ancient Greek poem describing the origins of the gods".to_string()),
    };
    zeus.metadata_mut().add_source(source.clone());
    hera.metadata_mut().add_source(source.clone());
    athena.metadata_mut().add_source(source);
    
    // Create family relationships
    let zeus_id = zeus.id().clone();
    let hera_id = hera.id().clone();
    let athena_id = athena.id().clone();
    
    // Zeus and Hera are spouses
    let zeus_hera_relationship = FamilyRelationship::new(
        "Marriage of Zeus and Hera",
        "The divine marriage of Zeus and Hera, king and queen of the Olympian gods.",
        zeus_id.clone(),
        hera_id.clone(),
        FamilyRelationshipType::Spouse
    );
    
    // Zeus is Athena's father
    let zeus_athena_relationship = FamilyRelationship::new(
        "Zeus fathers Athena",
        "Zeus is the father of Athena, who sprang fully formed from his head.",
        zeus_id.clone(),
        athena_id.clone(),
        FamilyRelationshipType::Parent
    );
    
    // Create Greek pantheon
    let mut olympian_pantheon = Pantheon::new(
        "Olympian Pantheon",
        "The principal deities in ancient Greek religion and mythology, residing atop Mount Olympus.",
        "Greek"
    );
    olympian_pantheon.add_primary_deity(zeus_id.clone());
    olympian_pantheon.add_primary_deity(hera_id.clone());
    olympian_pantheon.add_primary_deity(athena_id.clone());
    olympian_pantheon.set_cosmology("The cosmos is divided into three realms: the sky (Zeus), the sea (Poseidon), and the underworld (Hades).");
    
    // Create Greek culture
    let mut greek_culture = Culture::new(
        "Ancient Greek",
        "The culture of ancient Greece, which flourished from the archaic period to the end of antiquity."
    );
    greek_culture.add_region("Greece");
    greek_culture.add_region("Aegean");
    greek_culture.add_region("Ionia");
    greek_culture.add_region("Magna Graecia");
    greek_culture.add_language("Ancient Greek");
    
    // Add time periods
    greek_culture.add_time_period(TimePeriod {
        name: "Archaic Period".to_string(),
        start_year: Some(-800),
        end_year: Some(-480),
        description: Some("Period of ancient Greek history from the end of the Greek Dark Ages to the Persian Wars".to_string()),
    });
    
    greek_culture.add_time_period(TimePeriod {
        name: "Classical Period".to_string(),
        start_year: Some(-480),
        end_year: Some(-323),
        description: Some("Period of ancient Greek history from the Persian Wars to the death of Alexander the Great".to_string()),
    });
    
    // Add entities to ontology
    ontology.add_entity(zeus);
    ontology.add_entity(hera);
    ontology.add_entity(athena);
    ontology.add_entity(zeus_hera_relationship);
    ontology.add_entity(zeus_athena_relationship);
    ontology.add_entity(olympian_pantheon);
    ontology.add_entity(greek_culture);
    
    ontology
}

/// Create an example Norse pantheon with some deities
pub fn create_norse_example() -> MythOntology {
    let mut ontology = MythOntology::new();
    
    // Create Odin
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
    
    // Create Thor
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
    
    // Create Freyja
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
    
    // Add source to metadata
    let mut source = crate::core::Source {
        title: "Poetic Edda".to_string(),
        author: None,
        year: Some(1200),
        source_type: crate::core::SourceType::PrimaryText,
        url: None,
        notes: Some("Collection of Old Norse poems from the Icelandic medieval manuscript Codex Regius".to_string()),
    };
    odin.metadata_mut().add_source(source.clone());
    thor.metadata_mut().add_source(source.clone());
    freyja.metadata_mut().add_source(source);
    
    // Create family relationships
    let odin_id = odin.id().clone();
    let thor_id = thor.id().clone();
    
    // Odin is Thor's father
    let odin_thor_relationship = FamilyRelationship::new(
        "Odin fathers Thor",
        "Odin is the father of Thor, the god of thunder.",
        odin_id.clone(),
        thor_id.clone(),
        FamilyRelationshipType::Parent
    );
    
    // Create Norse pantheon
    let mut aesir_pantheon = Pantheon::new(
        "Aesir Pantheon",
        "The principal pantheon of gods in Norse mythology, associated with war, power, and governance.",
        "Norse"
    );
    aesir_pantheon.add_primary_deity(odin_id.clone());
    aesir_pantheon.add_primary_deity(thor_id.clone());
    aesir_pantheon.set_cosmology("The Norse cosmos is structured around Yggdrasil, the World Tree, which connects the nine worlds.");
    
    // Create Norse culture
    let mut norse_culture = Culture::new(
        "Norse",
        "The culture of the Norse people during the Viking Age."
    );
    norse_culture.add_region("Scandinavia");
    norse_culture.add_region("Iceland");
    norse_culture.add_language("Old Norse");
    
    // Add time period
    norse_culture.add_time_period(TimePeriod {
        name: "Viking Age".to_string(),
        start_year: Some(793),
        end_year: Some(1066),
        description: Some("Period in European history when Scandinavian Norsemen explored, raided, and settled throughout Europe".to_string()),
    });
    
    // Add entities to ontology
    ontology.add_entity(odin);
    ontology.add_entity(thor);
    ontology.add_entity(freyja);
    ontology.add_entity(odin_thor_relationship);
    ontology.add_entity(aesir_pantheon);
    ontology.add_entity(norse_culture);
    
    ontology
}
