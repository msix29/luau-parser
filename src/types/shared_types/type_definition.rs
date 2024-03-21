//! # Type Definition
//!
//! Module containg definition for type definitions.
//!

use crate::prelude::PossibleValues;

/// A struct holding values of a type, including it's `&` and `|` (intersection and union)
/// types.
#[derive(Clone, Debug, Default)]
pub struct TypeValue {
    /// The first value in the type, not the final type.
    pub r#type: Box<PossibleValues>,

    /// All types following the [main type](TypeValue::r#type) with `&` between them.
    pub and_types: Vec<Box<TypeDefinition>>,

    /// All types following the [main type](TypeValue::r#type) with `|` between them.
    pub or_types: Vec<Box<TypeDefinition>>,
}

/// A struct for a type definition. Holds needed data to be able to write it back as valid
/// luau.
#[derive(Clone, Debug)]
pub struct TypeDefinition {
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

    /// Whether or not this type was exported (had `export` keyword before it).
    /// ```lua
    /// export type Foo = { number }
    /// ```
    pub is_exported: bool,

    /// The _[actual definition](TypeValue)_ of the type.
    pub type_value: Box<TypeValue>,
}
