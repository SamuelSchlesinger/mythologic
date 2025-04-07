use crate::core::{MythOntology, MythEntity};
use crate::relationships::{
    FamilyRelationship, FamilyRelationshipType,
    ConflictRelationship, ConflictType, ConflictOutcome,
    AllianceRelationship, AllianceType,
    TransformationRelationship, TransformationType
};

/// Add relationships between Norse mythological entities
pub fn add_norse_relationships(ontology: &mut MythOntology) {
    // Family relationships
    add_family_relationships(ontology);
    
    // Conflict relationships
    add_conflict_relationships(ontology);
    
    // Alliance relationships
    add_alliance_relationships(ontology);
    
    // Transformation relationships
    add_transformation_relationships(ontology);
}

/// Add family relationships between Norse entities
fn add_family_relationships(ontology: &mut MythOntology) {
    // Find entity IDs
    let odin_id = find_entity_id(ontology, "Odin");
    let thor_id = find_entity_id(ontology, "Thor");
    let loki_id = find_entity_id(ontology, "Loki");
    let fenrir_id = find_entity_id(ontology, "Fenrir");
    let jormungandr_id = find_entity_id(ontology, "Jormungandr");
    
    // Odin is Thor's father
    if let (Some(odin_id), Some(thor_id)) = (odin_id.clone(), thor_id.clone()) {
        let odin_thor_relationship = FamilyRelationship::new(
            "Odin fathers Thor",
            "Odin is the father of Thor, the god of thunder.",
            odin_id,
            thor_id,
            FamilyRelationshipType::Parent
        );
        
        ontology.add_entity(MythEntity::FamilyRelationship(odin_thor_relationship));
    }
    
    // Loki is Fenrir's father
    if let (Some(loki_id), Some(fenrir_id)) = (loki_id.clone(), fenrir_id.clone()) {
        let loki_fenrir_relationship = FamilyRelationship::new(
            "Loki fathers Fenrir",
            "Loki is the father of the monstrous wolf Fenrir.",
            loki_id,
            fenrir_id,
            FamilyRelationshipType::Parent
        );
        
        ontology.add_entity(MythEntity::FamilyRelationship(loki_fenrir_relationship));
    }
    
    // Loki is Jormungandr's father
    if let (Some(loki_id), Some(jormungandr_id)) = (loki_id.clone(), jormungandr_id.clone()) {
        let loki_jormungandr_relationship = FamilyRelationship::new(
            "Loki fathers Jormungandr",
            "Loki is the father of Jormungandr, the World Serpent.",
            loki_id,
            jormungandr_id,
            FamilyRelationshipType::Parent
        );
        
        ontology.add_entity(MythEntity::FamilyRelationship(loki_jormungandr_relationship));
    }
}

/// Add conflict relationships in Norse mythology
fn add_conflict_relationships(ontology: &mut MythOntology) {
    // Find entity IDs
    let thor_id = find_entity_id(ontology, "Thor");
    let jormungandr_id = find_entity_id(ontology, "Jormungandr");
    let odin_id = find_entity_id(ontology, "Odin");
    let fenrir_id = find_entity_id(ontology, "Fenrir");
    
    // Thor vs Jormungandr at Ragnarök
    if let (Some(thor_id), Some(jormungandr_id)) = (thor_id, jormungandr_id) {
        let thor_jormungandr_conflict = ConflictRelationship::new(
            "Thor battles Jormungandr at Ragnarök",
            "At Ragnarök, Thor will fight and kill Jormungandr, but die from its venom.",
            thor_id.clone(),
            jormungandr_id.clone(),
            ConflictType::Battle
        );
        
        let mut thor_jormungandr_conflict = thor_jormungandr_conflict;
        thor_jormungandr_conflict.set_outcome(ConflictOutcome {
            description: "Thor slays Jormungandr but dies from its venom.".to_string(),
            victor_id: None, // Both die
            consequences: vec![
                "Thor walks nine steps before dying from the serpent's venom".to_string(),
                "The World Serpent is slain".to_string(),
            ],
        });
        
        ontology.add_entity(MythEntity::ConflictRelationship(thor_jormungandr_conflict));
    }
    
    // Odin vs Fenrir at Ragnarök
    if let (Some(odin_id), Some(fenrir_id)) = (odin_id, fenrir_id) {
        let odin_fenrir_conflict = ConflictRelationship::new(
            "Odin battles Fenrir at Ragnarök",
            "At Ragnarök, Odin will fight Fenrir and be devoured by the wolf.",
            odin_id.clone(),
            fenrir_id.clone(),
            ConflictType::Battle
        );
        
        let mut odin_fenrir_conflict = odin_fenrir_conflict;
        odin_fenrir_conflict.set_outcome(ConflictOutcome {
            description: "Fenrir swallows Odin whole.".to_string(),
            victor_id: Some(fenrir_id.clone()),
            consequences: vec![
                "Death of Odin".to_string(),
                "Vidar (Odin's son) later avenges his father by killing Fenrir".to_string(),
            ],
        });
        
        ontology.add_entity(MythEntity::ConflictRelationship(odin_fenrir_conflict));
    }
}

/// Add alliance relationships in Norse mythology
fn add_alliance_relationships(ontology: &mut MythOntology) {
    // Find entity IDs
    let odin_id = find_entity_id(ontology, "Odin");
    let thor_id = find_entity_id(ontology, "Thor");
    
    // Aesir alliance
    if let (Some(odin_id), Some(thor_id)) = (odin_id.clone(), thor_id.clone()) {
        let aesir_alliance = AllianceRelationship::new(
            "Aesir divine council",
            "Alliance of the Aesir gods under Odin's leadership.",
            odin_id,
            thor_id, // Representative of other Aesir
            AllianceType::Coalition,
            "Maintain cosmic order and defend against giants"
        );
        
        ontology.add_entity(MythEntity::AllianceRelationship(aesir_alliance));
    }
}

/// Add transformation relationships in Norse mythology
fn add_transformation_relationships(ontology: &mut MythOntology) {
    // Find entity IDs
    let loki_id = find_entity_id(ontology, "Loki");
    
    // Loki's shapeshifting
    if let Some(loki_id) = loki_id {
        let loki_transformation = TransformationRelationship::new(
            "Loki's shapeshifting",
            "Loki's ability to change his form at will into various creatures.",
            loki_id.clone(),
            loki_id.clone(), // Both source and target are Loki
            TransformationType::Shapeshifting,
            "Innate magical ability"
        );
        
        let mut loki_transformation = loki_transformation;
        loki_transformation.set_permanent(false);
        loki_transformation.set_reversible(true);
        
        ontology.add_entity(MythEntity::TransformationRelationship(loki_transformation));
    }
}

/// Helper function to find an entity ID by name
fn find_entity_id(ontology: &MythOntology, name: &str) -> Option<crate::core::MythId> {
    ontology.all_entities().iter()
        .find(|entity| entity.name() == name)
        .map(|entity| entity.id().clone())
}