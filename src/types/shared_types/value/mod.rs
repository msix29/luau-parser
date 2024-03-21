//! # Value
//!
//! Possible values to hold Luau datatypes.
//!

mod function;
mod simple;
mod table;

use std::fmt::Debug;

pub use function::*;
pub use simple::*;
pub use table::*;

use super::{SingleToken, TypeDefinition};

/// Enum representing one of the _major_ data types in Luau, being table, function, or
/// "simple". Simple is just anything that can be represented as a string, and in Luau,
/// that's just non-functions and non-tables!
#[derive(Clone, Debug)]
pub enum PossibleValues {
    /// A simple value.
    SimpleValue(SimpleValue),

    /// A function.
    FunctionValue(FunctionValue),

    /// A table.
    TableValue(TableValue),
}

/// Struct representing a possible value alongside the type it was casted to, if any.
#[derive(Clone, Debug, Default)]
pub struct Value {
    /// The actual value.
    pub value: PossibleValues,

    /// The `::` operator.
    pub operator: Option<SingleToken>,

    /// The type of the value, always `None` except if the value had a typecast (`::`).
    pub r#type: Option<TypeDefinition>,
}
