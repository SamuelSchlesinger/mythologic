use crate::core::{MythOntology, MythEntity};
use crate::relationships::{
    FamilyRelationship, FamilyRelationshipType,
    ConflictRelationship, ConflictType, ConflictOutcome,
    AllianceRelationship, AllianceType,
    TransformationRelationship, TransformationType
};

/// Add relationships between Greek mythological entities
pub fn add_greek_relationships(ontology: &mut MythOntology) {
    // Family relationships
    add_family_relationships(ontology);
    
    // Conflict relationships
    add_conflict_relationships(ontology);
    
    // Alliance relationships
    add_alliance_relationships(ontology);
    
    // Transformation relationships
    add_transformation_relationships(ontology);
}

/// Add family relationships between Greek deities and heroes
fn add_family_relationships(ontology: &mut MythOntology) {
    // Find entity IDs (normally we'd check if these exist first)
    let zeus_id = find_entity_id(ontology, "Zeus");
    let hera_id = find_entity_id(ontology, "Hera");
    let athena_id = find_entity_id(ontology, "Athena");
    let poseidon_id = find_entity_id(ontology, "Poseidon");
    let _apollo_id = find_entity_id(ontology, "Apollo");
    let heracles_id = find_entity_id(ontology, "Heracles");
    let perseus_id = find_entity_id(ontology, "Perseus");
    
    // Zeus and Hera are spouses
    if let (Some(zeus_id), Some(hera_id)) = (zeus_id.clone(), hera_id.clone()) {
        let zeus_hera_relationship = FamilyRelationship::new(
            "Marriage of Zeus and Hera",
            "The divine marriage of Zeus and Hera, king and queen of the Olympian gods.",
            zeus_id,
            hera_id,
            FamilyRelationshipType::Spouse
        );
        
        ontology.add_entity(MythEntity::FamilyRelationship(zeus_hera_relationship));
    }
    
    // Zeus is Athena's father
    if let (Some(zeus_id), Some(athena_id)) = (zeus_id.clone(), athena_id.clone()) {
        let zeus_athena_relationship = FamilyRelationship::new(
            "Zeus fathers Athena",
            "Zeus is the father of Athena, who sprang fully formed from his head.",
            zeus_id,
            athena_id,
            FamilyRelationshipType::Parent
        );
        
        ontology.add_entity(MythEntity::FamilyRelationship(zeus_athena_relationship));
    }
    
    // Zeus is Heracles' father
    if let (Some(zeus_id), Some(heracles_id)) = (zeus_id.clone(), heracles_id.clone()) {
        let zeus_heracles_relationship = FamilyRelationship::new(
            "Zeus fathers Heracles",
            "Zeus is the father of Heracles by the mortal woman Alcmene.",
            zeus_id,
            heracles_id,
            FamilyRelationshipType::Parent
        );
        
        ontology.add_entity(MythEntity::FamilyRelationship(zeus_heracles_relationship));
    }
    
    // Zeus is Perseus' father
    if let (Some(zeus_id), Some(perseus_id)) = (zeus_id.clone(), perseus_id.clone()) {
        let zeus_perseus_relationship = FamilyRelationship::new(
            "Zeus fathers Perseus",
            "Zeus is the father of Perseus by the mortal woman Danaë.",
            zeus_id,
            perseus_id,
            FamilyRelationshipType::Parent
        );
        
        ontology.add_entity(MythEntity::FamilyRelationship(zeus_perseus_relationship));
    }
    
    // Zeus, Poseidon, and Hades are siblings
    if let (Some(zeus_id), Some(poseidon_id)) = (zeus_id.clone(), poseidon_id.clone()) {
        let zeus_poseidon_relationship = FamilyRelationship::new(
            "Zeus and Poseidon brotherhood",
            "Zeus and Poseidon are brothers, sons of Cronos and Rhea.",
            zeus_id,
            poseidon_id,
            FamilyRelationshipType::Sibling
        );
        
        ontology.add_entity(MythEntity::FamilyRelationship(zeus_poseidon_relationship));
    }
}

/// Add conflict relationships in Greek mythology
fn add_conflict_relationships(ontology: &mut MythOntology) {
    // Find entity IDs
    let zeus_id = find_entity_id(ontology, "Zeus");
    let heracles_id = find_entity_id(ontology, "Heracles");
    let perseus_id = find_entity_id(ontology, "Perseus");
    let medusa_id = find_entity_id(ontology, "Medusa");
    
    // Zeus vs Titans (Titanomachy)
    if let Some(zeus_id) = zeus_id {
        let titanomachy = ConflictRelationship::new(
            "Titanomachy",
            "The ten-year war between the Olympian gods led by Zeus and the Titans led by Cronus.",
            zeus_id.clone(),
            zeus_id.clone(), // Placeholder as we don't have a Titan entity
            ConflictType::War
        );
        
        let mut titanomachy = titanomachy;
        titanomachy.set_outcome(ConflictOutcome {
            description: "The Olympians defeated the Titans.".to_string(),
            victor_id: Some(zeus_id.clone()),
            consequences: vec![
                "The Olympians became the ruling divine power".to_string(),
                "The Titans were imprisoned in Tartarus".to_string(),
                "Zeus became the ruler of the gods".to_string(),
            ],
        });
        
        ontology.add_entity(MythEntity::ConflictRelationship(titanomachy));
    }
    
    // Perseus vs Medusa
    if let (Some(perseus_id), Some(medusa_id)) = (perseus_id, medusa_id) {
        let perseus_medusa_conflict = ConflictRelationship::new(
            "Perseus slays Medusa",
            "Perseus's quest to slay the Gorgon Medusa.",
            perseus_id.clone(),
            medusa_id.clone(),
            ConflictType::Battle
        );
        
        let mut perseus_medusa_conflict = perseus_medusa_conflict;
        perseus_medusa_conflict.set_outcome(ConflictOutcome {
            description: "Perseus beheaded Medusa.".to_string(),
            victor_id: Some(perseus_id.clone()),
            consequences: vec![
                "Perseus gained Medusa's head as a weapon".to_string(),
                "Pegasus and Chrysaor sprang from Medusa's blood".to_string(),
            ],
        });
        
        ontology.add_entity(MythEntity::ConflictRelationship(perseus_medusa_conflict));
    }
    
    // Heracles' Labors (conflict with various monsters)
    if let Some(heracles_id) = heracles_id {
        let labors = ConflictRelationship::new(
            "Twelve Labors of Heracles",
            "Series of tasks assigned to Heracles by King Eurystheus as penance.",
            heracles_id.clone(),
            heracles_id.clone(), // Placeholder as we don't have monster entities
            ConflictType::Contest
        );
        
        let mut labors = labors;
        labors.set_outcome(ConflictOutcome {
            description: "Heracles completed all twelve labors.".to_string(),
            victor_id: Some(heracles_id.clone()),
            consequences: vec![
                "Heracles was purified of his sins".to_string(),
                "Heracles gained immortality and godhood".to_string(),
                "Heracles's reputation as a hero was cemented".to_string(),
            ],
        });
        
        ontology.add_entity(MythEntity::ConflictRelationship(labors));
    }
}

/// Add alliance relationships in Greek mythology
fn add_alliance_relationships(ontology: &mut MythOntology) {
    // Find entity IDs
    let zeus_id = find_entity_id(ontology, "Zeus");
    let athena_id = find_entity_id(ontology, "Athena");
    let perseus_id = find_entity_id(ontology, "Perseus");
    
    // Athena's patronage of Perseus
    if let (Some(athena_id), Some(perseus_id)) = (athena_id.clone(), perseus_id.clone()) {
        let athena_perseus_alliance = AllianceRelationship::new(
            "Athena's patronage of Perseus",
            "Athena's divine assistance and guidance to Perseus during his quest.",
            athena_id,
            perseus_id,
            AllianceType::Patronage,
            "Divine guidance and protection"
        );
        
        ontology.add_entity(MythEntity::AllianceRelationship(athena_perseus_alliance));
    }
    
    // Zeus and Olympians alliance
    if let (Some(zeus_id), Some(athena_id)) = (zeus_id.clone(), athena_id.clone()) {
        let olympian_alliance = AllianceRelationship::new(
            "Olympian divine council",
            "Alliance of the major Olympian deities under Zeus's leadership.",
            zeus_id,
            athena_id, // Representative of other Olympians
            AllianceType::Coalition,
            "Governance of cosmos and mortal affairs"
        );
        
        ontology.add_entity(MythEntity::AllianceRelationship(olympian_alliance));
    }
}

/// Add transformation relationships in Greek mythology
fn add_transformation_relationships(ontology: &mut MythOntology) {
    // Find entity IDs
    let medusa_id = find_entity_id(ontology, "Medusa");
    
    // Medusa's transformation
    if let Some(medusa_id) = medusa_id {
        let medusa_transformation = TransformationRelationship::new(
            "Transformation of Medusa",
            "Athena transformed the beautiful maiden Medusa into a monstrous Gorgon.",
            medusa_id.clone(),
            medusa_id.clone(), // Both source and target are Medusa
            TransformationType::Curse,
            "Punishment by Athena for desecration of her temple"
        );
        
        let mut medusa_transformation = medusa_transformation;
        medusa_transformation.set_permanent(true);
        medusa_transformation.set_reversible(false);
        
        ontology.add_entity(MythEntity::TransformationRelationship(medusa_transformation));
    }
}

/// Helper function to find an entity ID by name
fn find_entity_id(ontology: &MythOntology, name: &str) -> Option<crate::core::MythId> {
    ontology.all_entities().iter()
        .find(|entity| entity.name() == name)
        .map(|entity| entity.id().clone())
}