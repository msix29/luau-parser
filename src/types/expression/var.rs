use luau_lexer::prelude::Token;

use super::TableAccess;

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
    Name(Token),

    /// A field accessed from a table.
    ///
    /// ```lua
    /// local _ = t.foo
    /// ```
    TableAccess(TableAccess),
}
