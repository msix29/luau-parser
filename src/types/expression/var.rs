use luau_lexer::prelude::{Literal, LuauNumber, LuauString, Token};
use std::sync::Arc;

use super::{
    Ast, GenericDeclaration, List, NormalizedName, Table, TableAccess, TableKey, TypeValue,
};

/// Name of a [`variable`](Var).
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct VariableName {
    /// The actual token holding the name.
    pub token: Token,
}

/// Possible ways in which a variable can be referenced.
#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum Var {
    /// This [`Var`] had a syntax error.
    #[default]
    ERROR,

    /// A simple reference to the variable.
    ///
    /// ```lua
    /// local _ = foo
    /// ```
    Name(VariableName),

    /// A field accessed from a table.
    ///
    /// ```lua
    /// local _ = t.foo
    /// ```
    TableAccess(TableAccess),
}
