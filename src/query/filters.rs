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
    pub fn matches(&self, entity: &MythEntity) -> bool {
        match self {
            Self::EntityType(entity_type) => {
                entity.entity_type() == entity_type
            },
            Self::NameContains(substring) => {
                entity.name().to_lowercase().contains(&substring.to_lowercase())
            },
            Self::Culture(culture) => {
                if let Some(entity_culture) = entity.culture() {
                    entity_culture == culture
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
