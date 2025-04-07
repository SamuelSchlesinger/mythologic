use crate::core::{MythOntology, MythEntity};
use crate::cultural::{Culture, TimePeriod, MythologicalRegion};

/// Add Greek culture and related entities to the ontology
pub fn add_greek_culture(ontology: &mut MythOntology) {
    // Create Greek culture
    let mut greek_culture = Culture::new(
        "Ancient Greek",
        "The culture of ancient Greece, which flourished from the archaic period to the end of antiquity."
    );
    
    // Add regions
    greek_culture.add_region("Greece");
    greek_culture.add_region("Aegean");
    greek_culture.add_region("Ionia");
    greek_culture.add_region("Magna Graecia");
    
    // Add languages
    greek_culture.add_language("Ancient Greek");
    greek_culture.add_language("Koine Greek");
    
    // Add time periods
    greek_culture.add_time_period(TimePeriod {
        name: "Mycenaean Period".to_string(),
        start_year: Some(-1600),
        end_year: Some(-1100),
        description: Some("The period of Greek history at the end of the Bronze Age, before the Greek Dark Ages".to_string()),
    });
    
    greek_culture.add_time_period(TimePeriod {
        name: "Greek Dark Ages".to_string(),
        start_year: Some(-1100),
        end_year: Some(-800),
        description: Some("The period of Greek history from the collapse of the Mycenaean palaces to the beginning of the Archaic period".to_string()),
    });
    
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
    
    greek_culture.add_time_period(TimePeriod {
        name: "Hellenistic Period".to_string(),
        start_year: Some(-323),
        end_year: Some(-31),
        description: Some("Period of ancient Greek history from the death of Alexander the Great to the Battle of Actium".to_string()),
    });
    
    // Add cultural practices
    greek_culture.add_cultural_practice("Olympic Games");
    greek_culture.add_cultural_practice("Theater festivals");
    greek_culture.add_cultural_practice("Symposia");
    greek_culture.add_cultural_practice("Religious festivals");
    
    // Add Greek regions as separate entities
    let greece_region = MythologicalRegion::new(
        "Greece",
        "The heartland of Greek civilization, including mainland Greece and the Peloponnese.",
        None
    );
    
    let aegean_region = MythologicalRegion::new(
        "Aegean",
        "The Aegean Sea and its islands, including the Cyclades, Dodecanese, and Crete.",
        None
    );
    
    let ionia_region = MythologicalRegion::new(
        "Ionia",
        "The coastal region of western Anatolia (modern Turkey) settled by Greeks.",
        None
    );
    
    let magna_graecia_region = MythologicalRegion::new(
        "Magna Graecia",
        "Greek colonies in southern Italy and Sicily.",
        None
    );
    
    // Add to ontology
    ontology.add_entity(MythEntity::Culture(greek_culture));
    ontology.add_entity(MythEntity::MythologicalRegion(greece_region));
    ontology.add_entity(MythEntity::MythologicalRegion(aegean_region));
    ontology.add_entity(MythEntity::MythologicalRegion(ionia_region));
    ontology.add_entity(MythEntity::MythologicalRegion(magna_graecia_region));
}