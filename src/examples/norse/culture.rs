use crate::core::{MythOntology, MythEntity};
use crate::cultural::{Culture, TimePeriod, MythologicalRegion};

/// Add Norse culture and related entities to the ontology
pub fn add_norse_culture(ontology: &mut MythOntology) {
    // Create Norse culture
    let mut norse_culture = Culture::new(
        "Norse",
        "The culture of the Norse people during the Viking Age."
    );
    
    // Add regions
    norse_culture.add_region("Scandinavia");
    norse_culture.add_region("Iceland");
    norse_culture.add_region("Greenland");
    norse_culture.add_region("Faroe Islands");
    
    // Add languages
    norse_culture.add_language("Old Norse");
    norse_culture.add_language("Old Icelandic");
    
    // Add time periods
    norse_culture.add_time_period(TimePeriod {
        name: "Germanic Iron Age".to_string(),
        start_year: Some(400),
        end_year: Some(800),
        description: Some("Period before the Viking Age with early development of Norse culture".to_string()),
    });
    
    norse_culture.add_time_period(TimePeriod {
        name: "Viking Age".to_string(),
        start_year: Some(793),
        end_year: Some(1066),
        description: Some("Period in European history when Scandinavian Norsemen explored, raided, and settled throughout Europe".to_string()),
    });
    
    norse_culture.add_time_period(TimePeriod {
        name: "Christianization".to_string(),
        start_year: Some(1000),
        end_year: Some(1300),
        description: Some("Period when Norse regions converted to Christianity and traditional belief systems declined".to_string()),
    });
    
    // Add cultural practices
    norse_culture.add_cultural_practice("Blót (seasonal sacrifices)");
    norse_culture.add_cultural_practice("Sumbel (ritual drinking ceremony)");
    norse_culture.add_cultural_practice("Sailing and navigation");
    norse_culture.add_cultural_practice("Rune carving");
    norse_culture.add_cultural_practice("Storytelling and poetry");
    
    // Add Norse regions as separate entities
    let scandinavia_region = MythologicalRegion::new(
        "Scandinavia",
        "The northern European region encompassing modern-day Norway, Sweden, and Denmark.",
        None
    );
    
    let iceland_region = MythologicalRegion::new(
        "Iceland",
        "Island in the North Atlantic settled by Norse people in the 9th century.",
        None
    );
    
    let greenland_region = MythologicalRegion::new(
        "Greenland",
        "Large island in the North Atlantic colonized by Norse settlers from Iceland.",
        None
    );
    
    // Add to ontology
    ontology.add_entity(MythEntity::Culture(norse_culture));
    ontology.add_entity(MythEntity::MythologicalRegion(scandinavia_region));
    ontology.add_entity(MythEntity::MythologicalRegion(iceland_region));
    ontology.add_entity(MythEntity::MythologicalRegion(greenland_region));
}