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

/// Enum representing one of the _major_ data types in Luau, being table, function, or
/// "simple". Simple is just anything that can be represented as a string, and in Luau,
/// that's just non-functions and non-tables!
#[derive(Clone, Debug)]
pub enum Value {
    /// A simple value.
    SimpleValue(SimpleValue),

    /// A function.
    FunctionValue(FunctionValue),

    /// A table.
    TableValue(TableValue),
}
