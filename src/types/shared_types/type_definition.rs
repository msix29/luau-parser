//! # Type Definition
//!
//! Module containg definition for type definitions.
//!

use std::sync::Arc;

use crate::prelude::Expression;

use super::{Location, SingleToken};

/// A struct holding values of a type, including it's `&` and `|` (intersection and union)
/// types.
#[derive(Clone, Debug, Default)]
pub struct TypeValue {
    /// The first value in the type, not the final type.
    pub r#type: Arc<Expression>,

    /// All types following the [main type](TypeValue::r#type) with `&` between them.
    pub and_types: Vec<Arc<TypeDefinition>>,

    /// All types following the [main type](TypeValue::r#type) with `|` between them.
    pub or_types: Vec<Arc<TypeDefinition>>,
}

/// A struct for a type definition. Holds needed data to be able to write it back as valid
/// luau.
#[derive(Clone, Debug)]
pub struct TypeDefinition {
    /// The `export` keyword.
    pub export_keyword: Option<SingleToken>,

    /// The `type` keyword.
    pub type_keyword: Option<SingleToken>,

    /// The name of the type. Will always be an empty string if this is a type with no
    /// prior definition, like:
    ///
    /// ```lua
    /// local foo: { number }
    ///
    /// local function bar(qux: () -> ())
    /// end
    /// ```
    ///
    /// In the 3 cases (`foo`, `bar`, and `qux`), they all have types with no names.
    pub type_name: String,

    /// Exact location of the type name.
    pub name_location: Option<Location>,

    /// The `=` sign between the name and the actual value of the type.
    pub equal_sign: Option<SingleToken>,

    /// The _[actual definition](TypeValue)_ of the type.
    pub type_value: Arc<TypeValue>,

    /// Exact location of the type definition.
    pub location: Location,
}
