//! # Type Definition
//!
//! Module containg definition for type definitions.
//!

use std::sync::Arc;

use crate::prelude::Expression;

use super::{FunctionParameter, List, Location, NormalizedName, SingleToken, TableField, TableValue};

// /// A struct holding values of a type, including it's `&` and `|` (intersection and union)
// /// types.
// #[derive(Clone, Debug, Default)]
// pub struct TypeValue {
//     /// The first value in the type, not the final type.
//     pub r#type: Arc<Expression>,

//     /// All types following the [main type](TypeValue::r#type) with `&` between them.
//     pub and_types: Vec<Arc<TypeDefinition>>,

//     /// All types following the [main type](TypeValue::r#type) with `|` between them.
//     pub or_types: Vec<Arc<TypeDefinition>>,
// }

#[derive(Clone, Debug)]
pub enum TypeValue {
    Array {
        opening_braces: SingleToken,
        type_info: Arc<TypeValue>,
        closing_braces: SingleToken,
    },

    Basic(SingleToken),

    String(SingleToken),

    Boolean(SingleToken),

    Function {
        opening_parentheses: SingleToken,
        arguments: List<FunctionParameter>,
        closing_parentheses: SingleToken,
        arrow: SingleToken,
        return_type: Arc<TypeValue>,
    },

    GenericPack {
        name: SingleToken,
        ellipse: SingleToken,
    },

    Intersection {
        left: Arc<TypeValue>,
        ampersand: SingleToken,
        right: Arc<TypeValue>,
    },

    Module {
        module: SingleToken,
        dot: SingleToken,
        //TODO: Allow generics
        type_info: Arc<SingleToken>,
    },

    Optional {
        base: Arc<TypeValue>,
        question_mark: SingleToken,
    },

    Table(TableValue),

    Typeof {
        typeof_token: SingleToken,
        opening_parentheses: SingleToken,
        inner: Arc<Expression>,
        closing_parentheses: SingleToken,
    },

    Tuple {
        opening_parentheses: SingleToken,
        types: List<TypeValue>,
        closing_parentheses: SingleToken,
    },

    Union {
        left: Arc<TypeValue>,
        pipe: SingleToken,
        right: Arc<TypeValue>,
    },

    Variadic {
        ellipse: SingleToken,
        type_info: Arc<TypeValue>,
    },

    VariadicPack {
        ellipse: SingleToken,
        name: SingleToken,
    },
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
}
