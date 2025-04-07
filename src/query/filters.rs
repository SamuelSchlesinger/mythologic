//! # Query Filtering System
//!
//! This module provides a flexible query filtering system for mythological entities.
//! It allows for building complex search criteria through a composable filter API.
//! 
//! Key features:
//! - Simple filter types (entity type, name, culture, etc.)
//! - Logical operations (AND, OR, NOT)
//! - Type-safe culture-based filtering using `CultureId`
//! - Case-insensitive partial name matching

use crate::core::{MythEntity, CultureId};

/// A filter for querying mythological entities.
///
/// `QueryFilter` provides a composable way to express search criteria for
/// mythological entities. It supports filtering by various attributes and
/// can be combined using logical operators to create complex queries.
///
/// # Examples
///
/// ```
/// use mythologic::query::QueryFilter;
/// use mythologic::core::MythEntity;
///
/// // Find Greek deities whose names contain "zeus"
/// let filter = QueryFilter::And(
///     Box::new(QueryFilter::EntityType("Deity".to_string())),
///     Box::new(QueryFilter::And(
///         Box::new(QueryFilter::Culture("Greek".into())),
///         Box::new(QueryFilter::NameContains("zeus".to_string()))
///     ))
/// );
///
/// // Usage with a collection of entities
/// // entities.iter().filter(|e| filter.matches(e)).collect::<Vec<_>>();
/// ```
pub enum QueryFilter {
    /// Filter by entity type (e.g., "Deity", "Hero", "Artifact").
    ///
    /// Matches entities whose `entity_type()` method returns exactly the specified string.
    EntityType(String),
    
    /// Filter by partial case-insensitive name match.
    ///
    /// Matches entities whose name contains the provided substring (case-insensitive).
    NameContains(String),
    
    /// Filter by cultural origin.
    ///
    /// Matches entities that belong to the specified culture.
    Culture(CultureId),
    
    /// Filter by the presence of a metadata attribute.
    ///
    /// Matches entities that have the specified attribute key in their metadata.
    HasAttribute(String),
    
    /// Filter by metadata attribute value.
    ///
    /// Matches entities that have the specified attribute key with exactly the specified value.
    AttributeEquals(String, String),
    
    /// Logical AND of two filters.
    ///
    /// Matches entities that match both the left and right filters.
    And(Box<QueryFilter>, Box<QueryFilter>),
    
    /// Logical OR of two filters.
    ///
    /// Matches entities that match either the left or right filter.
    Or(Box<QueryFilter>, Box<QueryFilter>),
    
    /// Logical NOT of a filter.
    ///
    /// Matches entities that do not match the provided filter.
    Not(Box<QueryFilter>),
}

impl QueryFilter {
    /// Check if an entity matches this filter.
    ///
    /// This method evaluates the filter against a given entity and returns
    /// true if the entity satisfies the filter criteria.
    ///
    /// # Arguments
    ///
    /// * `entity` - The entity to check against this filter
    ///
    /// # Returns
    ///
    /// `true` if the entity matches the filter, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use mythologic::query::QueryFilter;
    /// use mythologic::core::MythEntity;
    ///
    /// let name_filter = QueryFilter::NameContains("zeus".to_string());
    /// 
    /// // Assuming you have an entity in scope:
    /// // if name_filter.matches(&zeus_entity) {
    /// //     println!("Found Zeus!");
    /// // }
    /// ```
    pub fn matches(&self, entity: &MythEntity) -> bool {
        match self {
            // Match by entity type (exact match)
            Self::EntityType(entity_type) => {
                entity.entity_type() == entity_type
            },
            
            // Match by name (case-insensitive substring)
            Self::NameContains(substring) => {
                entity.name().to_lowercase().contains(&substring.to_lowercase())
            },
            
            // Match by culture (exact match on CultureId)
            Self::Culture(culture) => {
                if let Some(entity_culture) = entity.culture() {
                    entity_culture == culture
                } else {
                    false
                }
            },
            
            // Match by attribute existence
            Self::HasAttribute(key) => {
                entity.metadata().attributes.contains_key(key)
            },
            
            // Match by attribute value (exact match)
            Self::AttributeEquals(key, value) => {
                entity.metadata().attributes.get(key).map_or(false, |v| v == value)
            },
            
            // Logical operators
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
    
    /// Creates a filter that combines this filter with another using AND logic.
    ///
    /// # Arguments
    ///
    /// * `other` - The filter to combine with this one
    ///
    /// # Returns
    ///
    /// A new filter that matches only if both this filter and the other filter match.
    ///
    /// # Examples
    ///
    /// ```
    /// use mythologic::query::QueryFilter;
    ///
    /// let greek = QueryFilter::Culture("Greek".into());
    /// let deity = QueryFilter::EntityType("Deity".to_string());
    /// 
    /// // Filter for Greek deities
    /// let greek_deities = greek.and(deity);
    /// ```
    pub fn and(self, other: QueryFilter) -> QueryFilter {
        QueryFilter::And(Box::new(self), Box::new(other))
    }
    
    /// Creates a filter that combines this filter with another using OR logic.
    ///
    /// # Arguments
    ///
    /// * `other` - The filter to combine with this one
    ///
    /// # Returns
    ///
    /// A new filter that matches if either this filter or the other filter match.
    ///
    /// # Examples
    ///
    /// ```
    /// use mythologic::query::QueryFilter;
    ///
    /// let greek = QueryFilter::Culture("Greek".into());
    /// let norse = QueryFilter::Culture("Norse".into());
    /// 
    /// // Filter for either Greek or Norse entities
    /// let greek_or_norse = greek.or(norse);
    /// ```
    pub fn or(self, other: QueryFilter) -> QueryFilter {
        QueryFilter::Or(Box::new(self), Box::new(other))
    }
    
    /// Creates a filter that negates this filter.
    ///
    /// # Returns
    ///
    /// A new filter that matches only if this filter does not match.
    ///
    /// # Examples
    ///
    /// ```
    /// use mythologic::query::QueryFilter;
    ///
    /// let greek = QueryFilter::Culture("Greek".into());
    /// 
    /// // Filter for non-Greek entities
    /// let non_greek = greek.not();
    /// ```
    pub fn not(self) -> QueryFilter {
        QueryFilter::Not(Box::new(self))
    }
}

/// Allow creating a `Culture` filter directly from a string.
///
/// This implementation makes it convenient to create culture filters without
/// explicitly constructing a `CultureId`.
///
/// # Examples
///
/// ```
/// use mythologic::query::QueryFilter;
///
/// // These are equivalent:
/// let filter1 = QueryFilter::Culture("Greek".into());
/// let filter2: QueryFilter = "Greek".into();
/// ```
impl From<&str> for QueryFilter {
    fn from(culture_name: &str) -> Self {
        QueryFilter::Culture(CultureId::new(culture_name))
    }
}
