use crate::core::{MythId, Metadata, CultureId};
use crate::entities::{Deity, Hero, Creature, Artifact, Location, Concept};
use crate::relationships::{Relationship, FamilyRelationship, AllianceRelationship, ConflictRelationship, TransformationRelationship};
use crate::cultural::{Culture, Pantheon, MythologicalEra, MythologicalRegion};

/// An enum representing any entity in the mythological ontology
#[derive(Clone)]
pub enum MythEntity {
    // Entities
    Deity(Deity),
    Hero(Hero),
    Creature(Creature),
    Artifact(Artifact),
    Location(Location),
    Concept(Concept),
    
    // Relationships
    Relationship(Relationship),
    FamilyRelationship(FamilyRelationship),
    AllianceRelationship(AllianceRelationship),
    ConflictRelationship(ConflictRelationship),
    TransformationRelationship(TransformationRelationship),
    
    // Cultural contexts
    Culture(Culture),
    Pantheon(Pantheon),
    MythologicalEra(MythologicalEra),
    MythologicalRegion(MythologicalRegion),
}

impl MythEntity {
    /// Get the unique identifier for this entity
    pub fn id(&self) -> &MythId {
        match self {
            Self::Deity(e) => &e.id,
            Self::Hero(e) => &e.id,
            Self::Creature(e) => &e.id,
            Self::Artifact(e) => &e.id,
            Self::Location(e) => &e.id,
            Self::Concept(e) => &e.id,
            Self::Relationship(e) => &e.id,
            Self::FamilyRelationship(e) => &e.relationship.id,
            Self::AllianceRelationship(e) => &e.relationship.id,
            Self::ConflictRelationship(e) => &e.relationship.id,
            Self::TransformationRelationship(e) => &e.relationship.id,
            Self::Culture(e) => &e.id,
            Self::Pantheon(e) => &e.id,
            Self::MythologicalEra(e) => &e.id,
            Self::MythologicalRegion(e) => &e.id,
        }
    }
    
    /// Get the name of this entity
    pub fn name(&self) -> &str {
        match self {
            Self::Deity(e) => &e.name,
            Self::Hero(e) => &e.name,
            Self::Creature(e) => &e.name,
            Self::Artifact(e) => &e.name,
            Self::Location(e) => &e.name,
            Self::Concept(e) => &e.name,
            Self::Relationship(e) => &e.name,
            Self::FamilyRelationship(e) => &e.relationship.name,
            Self::AllianceRelationship(e) => &e.relationship.name,
            Self::ConflictRelationship(e) => &e.relationship.name,
            Self::TransformationRelationship(e) => &e.relationship.name,
            Self::Culture(e) => &e.name,
            Self::Pantheon(e) => &e.name,
            Self::MythologicalEra(e) => &e.name,
            Self::MythologicalRegion(e) => &e.name,
        }
    }
    
    /// Get the metadata for this entity
    pub fn metadata(&self) -> &Metadata {
        match self {
            Self::Deity(e) => &e.metadata,
            Self::Hero(e) => &e.metadata,
            Self::Creature(e) => &e.metadata,
            Self::Artifact(e) => &e.metadata,
            Self::Location(e) => &e.metadata,
            Self::Concept(e) => &e.metadata,
            Self::Relationship(e) => &e.metadata,
            Self::FamilyRelationship(e) => &e.relationship.metadata,
            Self::AllianceRelationship(e) => &e.relationship.metadata,
            Self::ConflictRelationship(e) => &e.relationship.metadata,
            Self::TransformationRelationship(e) => &e.relationship.metadata,
            Self::Culture(e) => &e.metadata,
            Self::Pantheon(e) => &e.metadata,
            Self::MythologicalEra(e) => &e.metadata,
            Self::MythologicalRegion(e) => &e.metadata,
        }
    }
    
    /// Get mutable metadata for this entity
    pub fn metadata_mut(&mut self) -> &mut Metadata {
        match self {
            Self::Deity(e) => &mut e.metadata,
            Self::Hero(e) => &mut e.metadata,
            Self::Creature(e) => &mut e.metadata,
            Self::Artifact(e) => &mut e.metadata,
            Self::Location(e) => &mut e.metadata,
            Self::Concept(e) => &mut e.metadata,
            Self::Relationship(e) => &mut e.metadata,
            Self::FamilyRelationship(e) => &mut e.relationship.metadata,
            Self::AllianceRelationship(e) => &mut e.relationship.metadata,
            Self::ConflictRelationship(e) => &mut e.relationship.metadata,
            Self::TransformationRelationship(e) => &mut e.relationship.metadata,
            Self::Culture(e) => &mut e.metadata,
            Self::Pantheon(e) => &mut e.metadata,
            Self::MythologicalEra(e) => &mut e.metadata,
            Self::MythologicalRegion(e) => &mut e.metadata,
        }
    }
    
    /// Get the entity type as a string
    pub fn entity_type(&self) -> &'static str {
        match self {
            Self::Deity(_) => "Deity",
            Self::Hero(_) => "Hero",
            Self::Creature(_) => "Creature",
            Self::Artifact(_) => "Artifact",
            Self::Location(_) => "Location",
            Self::Concept(_) => "Concept",
            Self::Relationship(_) => "Relationship",
            Self::FamilyRelationship(_) => "FamilyRelationship",
            Self::AllianceRelationship(_) => "AllianceRelationship",
            Self::ConflictRelationship(_) => "ConflictRelationship",
            Self::TransformationRelationship(_) => "TransformationRelationship",
            Self::Culture(_) => "Culture",
            Self::Pantheon(_) => "Pantheon",
            Self::MythologicalEra(_) => "MythologicalEra",
            Self::MythologicalRegion(_) => "MythologicalRegion",
        }
    }
    
    /// Get the relationships this entity has
    pub fn relationships(&self) -> Vec<MythId> {
        match self {
            Self::Deity(e) => e.relationships.clone(),
            Self::Hero(e) => e.relationships.clone(),
            Self::Creature(e) => e.relationships.clone(),
            Self::Artifact(e) => e.relationships.clone(),
            Self::Location(e) => e.relationships.clone(),
            Self::Concept(e) => e.relationships.clone(),
            Self::Culture(e) => e.relationships.clone(),
            Self::Pantheon(e) => e.relationships.clone(),
            Self::MythologicalEra(e) => e.relationships.clone(),
            Self::MythologicalRegion(e) => e.relationships.clone(),
            _ => Vec::new(),
        }
    }
    
    /// Get the culture of this entity if applicable
    pub fn culture(&self) -> Option<&CultureId> {
        match self {
            Self::Deity(e) => Some(&e.culture),
            Self::Hero(e) => Some(&e.culture),
            Self::Creature(e) => Some(&e.culture),
            Self::Artifact(e) => Some(&e.culture),
            Self::Location(e) => Some(&e.culture),
            Self::Concept(e) => Some(&e.culture),
            Self::Pantheon(e) => Some(&e.culture),
            Self::MythologicalEra(e) => Some(&e.culture),
            _ => None,
        }
    }
    
    /// Get the culture name as a string if applicable
    pub fn culture_name(&self) -> Option<&str> {
        self.culture().map(|c| c.value())
    }
}
