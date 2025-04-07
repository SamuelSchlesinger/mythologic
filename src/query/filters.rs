use crate::core::MythEntity;

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
                // Try to get culture from various entity types
                if let Some(deity) = entity.downcast_ref::<crate::entities::Deity>() {
                    deity.culture() == culture
                } else if let Some(hero) = entity.downcast_ref::<crate::entities::Hero>() {
                    hero.culture() == culture
                } else if let Some(creature) = entity.downcast_ref::<crate::entities::Creature>() {
                    creature.culture() == culture
                } else if let Some(artifact) = entity.downcast_ref::<crate::entities::Artifact>() {
                    artifact.culture() == culture
                } else if let Some(location) = entity.downcast_ref::<crate::entities::Location>() {
                    location.culture() == culture
                } else if let Some(concept) = entity.downcast_ref::<crate::entities::Concept>() {
                    concept.culture() == culture
                } else {
                    false
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
