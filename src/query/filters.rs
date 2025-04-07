use crate::core::MythEntity;
use crate::entities::{Deity, Hero, Creature, Artifact, Location, Concept};

/// A filter for querying mythological entities
pub enum QueryFilter {
    /// Filter by entity type
    EntityType(String),
    /// Filter by name (partial match)
    NameContains(String),
    /// Filter by culture
    Culture(String),
    /// Filter by attribute presence
    HasAttribute(String),
    /// Filter by attribute value
    AttributeEquals(String, String),
    /// Combine filters with AND
    And(Box<QueryFilter>, Box<QueryFilter>),
    /// Combine filters with OR
    Or(Box<QueryFilter>, Box<QueryFilter>),
    /// Negate a filter
    Not(Box<QueryFilter>),
}

impl QueryFilter {
    // Helper methods to safely cast to specific entity types
    fn as_deity<'a>(&self, entity: &'a dyn MythEntity) -> Option<&'a Deity> {
        if entity.entity_type() == "Deity" {
            // This is unsafe but necessary since we can't use downcast_ref
            // We're checking the entity_type first to ensure safety
            unsafe {
                let ptr = entity as *const dyn MythEntity as *const Deity;
                Some(&*ptr)
            }
        } else {
            None
        }
    }

    fn as_hero<'a>(&self, entity: &'a dyn MythEntity) -> Option<&'a Hero> {
        if entity.entity_type() == "Hero" {
            unsafe {
                let ptr = entity as *const dyn MythEntity as *const Hero;
                Some(&*ptr)
            }
        } else {
            None
        }
    }

    fn as_creature<'a>(&self, entity: &'a dyn MythEntity) -> Option<&'a Creature> {
        if entity.entity_type() == "Creature" {
            unsafe {
                let ptr = entity as *const dyn MythEntity as *const Creature;
                Some(&*ptr)
            }
        } else {
            None
        }
    }

    fn as_artifact<'a>(&self, entity: &'a dyn MythEntity) -> Option<&'a Artifact> {
        if entity.entity_type() == "Artifact" {
            unsafe {
                let ptr = entity as *const dyn MythEntity as *const Artifact;
                Some(&*ptr)
            }
        } else {
            None
        }
    }

    fn as_location<'a>(&self, entity: &'a dyn MythEntity) -> Option<&'a Location> {
        if entity.entity_type() == "Location" {
            unsafe {
                let ptr = entity as *const dyn MythEntity as *const Location;
                Some(&*ptr)
            }
        } else {
            None
        }
    }

    fn as_concept<'a>(&self, entity: &'a dyn MythEntity) -> Option<&'a Concept> {
        if entity.entity_type() == "Concept" {
            unsafe {
                let ptr = entity as *const dyn MythEntity as *const Concept;
                Some(&*ptr)
            }
        } else {
            None
        }
    }
}

impl QueryFilter {
    /// Check if an entity matches this filter
    pub fn matches(&self, entity: &dyn MythEntity) -> bool {
        match self {
            Self::EntityType(entity_type) => {
                entity.entity_type() == entity_type
            },
            Self::NameContains(substring) => {
                entity.name().to_lowercase().contains(&substring.to_lowercase())
            },
            Self::Culture(culture) => {
                // We can't use downcast_ref with trait objects, so we'll check the entity type
                // and then try to extract the culture information
                match entity.entity_type() {
                    "Deity" => {
                        // For each entity type, we need to check if it matches what we're looking for
                        if let Some(deity) = self.as_deity(entity) {
                            deity.culture() == culture
                        } else {
                            false
                        }
                    },
                    "Hero" => {
                        if let Some(hero) = self.as_hero(entity) {
                            hero.culture() == culture
                        } else {
                            false
                        }
                    },
                    "Creature" => {
                        if let Some(creature) = self.as_creature(entity) {
                            creature.culture() == culture
                        } else {
                            false
                        }
                    },
                    "Artifact" => {
                        if let Some(artifact) = self.as_artifact(entity) {
                            artifact.culture() == culture
                        } else {
                            false
                        }
                    },
                    "Location" => {
                        if let Some(location) = self.as_location(entity) {
                            location.culture() == culture
                        } else {
                            false
                        }
                    },
                    "Concept" => {
                        if let Some(concept) = self.as_concept(entity) {
                            concept.culture() == culture
                        } else {
                            false
                        }
                    },
                    _ => false,
                }
            },
            Self::HasAttribute(key) => {
                entity.metadata().attributes.contains_key(key)
            },
            Self::AttributeEquals(key, value) => {
                entity.metadata().attributes.get(key).map_or(false, |v| v == value)
            },
            Self::And(left, right) => {
                left.matches(entity) && right.matches(entity)
            },
            Self::Or(left, right) => {
                left.matches(entity) || right.matches(entity)
            },
            Self::Not(filter) => {
                !filter.matches(entity)
            },
        }
    }
}
