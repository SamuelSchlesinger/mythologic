# Mythologic

A Rust library for modeling global mythological structures, their relationships, and how they interplay across different cultures and time periods.

[![Crates.io](https://img.shields.io/crates/v/mythologic.svg)](https://crates.io/crates/mythologic)
[![Documentation](https://docs.rs/mythologic/badge.svg)](https://docs.rs/mythologic)
[![MIT License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

## Overview

The `mythologic` crate provides a type-safe and extensible system for representing mythological concepts from various cultures throughout human history. It enables:

- Modeling of mythological entities (deities, heroes, creatures, artifacts, locations, concepts)
- Cultural contexts and historical eras
- Complex relationships between entities (family, alliance, conflict, transformation)
- Type-safe identifiers for referencing related concepts
- Querying and filtering of mythological data
- Visualization of mythological relationships and structures

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
mythologic = "0.1.0"
```

## Architecture

The crate is organized into several key modules:

- `core`: Core abstractions, traits, identifiers, and type definitions
- `entities`: Concrete mythological entities like deities, heroes, and creatures
- `cultural`: Cultural contexts such as pantheons, regions, and historical eras
- `relationships`: Relationships between mythological entities
- `query`: Query engine for filtering and retrieving mythological data
- `utils`: Utility functions and helpers
- `examples`: Comprehensive mythological examples from various cultures

## Usage Examples

### Creating Mythological Entities

```rust
use mythologic::entities::{Deity, Gender, DeityImportance};

// Create a new deity
let mut zeus = Deity::new("Zeus", "King of the Olympian gods", "Greek");

// Add details
zeus.add_domain("Sky");
zeus.add_domain("Thunder");
zeus.add_alternative_name("Jupiter");
zeus.set_gender(Gender::Male);
zeus.set_importance(DeityImportance::Supreme);
zeus.set_pantheon("Olympian");
```

### Defining Relationships

```rust
use mythologic::entities::{Deity, Hero};
use mythologic::relationships::{Family, FamilyRelation};

// Create entities
let zeus = Deity::new("Zeus", "King of the gods", "Greek");
let hercules = Hero::new("Hercules", "Son of Zeus, known for his strength", "Greek");

// Create a family relationship
let zeus_hercules = Family::new(
    zeus.id().clone(),
    hercules.id().clone(),
    FamilyRelation::Parent
);
```

### Querying Mythological Data

```rust
use mythologic::query::{QueryEngine, EntityFilter, AttributeFilter};
use mythologic::examples::create_greek_ontology;

// Get pre-built Greek mythology ontology
let greek_ontology = create_greek_ontology();

// Create a query engine
let query_engine = QueryEngine::new(&greek_ontology);

// Find all deities associated with the sky
let sky_deities = query_engine.query(
    EntityFilter::IsDeity,
    AttributeFilter::HasDomain("Sky")
);

// Find all heroes who are demigods
let demigods = query_engine.query(
    EntityFilter::IsHero,
    AttributeFilter::HasAttribute("Demigod", "true")
);
```

### Working with Complete Mythologies

The library includes pre-built comprehensive mythologies:

```rust
use mythologic::examples::{
    create_greek_ontology,
    create_norse_ontology,
    create_egyptian_ontology,
    create_celtic_ontology,
    create_hindu_ontology
};

// Load complete mythological ontologies
let greek = create_greek_ontology();
let norse = create_norse_ontology();
let egyptian = create_egyptian_ontology();
```

### Visualization Tool

The library includes a built-in visualization tool to explore mythological structures:

```bash
# Generate visualization for Greek mythology
mythologic_explorer greek

# Generate visualization for Norse mythology with custom output path
mythologic_explorer norse ./norse_visualization.html

# Generate visualizations for all available mythologies
mythologic_explorer all ./visualizations/
```

## Type Safety

This library uses the Rust type system to provide strong guarantees about data integrity. Instead of using raw strings for identifiers, domain-specific newtype wrappers are used (e.g., `CultureId`, `DeityId`, etc.) to ensure that:

1. References are always of the correct type
2. Errors in referencing become compile-time rather than runtime issues
3. Type-directed auto-completion works effectively
4. Domain-specific validation can be added as needed

## Available Mythologies

The library includes comprehensive examples for:

- Greek mythology (Olympians, heroes, creatures, locations)
- Norse mythology (Aesir, Vanir, Jotnar, Ragnarok cycle)
- Egyptian mythology (Ennead, pharaonic deities)
- Celtic mythology (Tuatha Dé Danann, Ulster cycle)
- Hindu mythology (Trimurti, Devas, Asuras)

Additionally, there are cross-cultural collections of:
- Mythological artifacts (weapons, magical items)
- Heroes and legendary figures
- Mythological creatures and monsters
- Sacred locations and realms
- Abstract concepts (fate, rebirth, cosmic order)

## Contributing

Contributions are welcome! Areas for expansion include:

1. Additional cultural mythologies (Mesopotamian, Mesoamerican, Chinese, etc.)
2. Enhanced relationship modeling
3. Improved visualization options
4. Integration with external data sources
5. Performance optimizations for large mythological datasets

## License

This project is licensed under the MIT License - see the LICENSE file for details.
