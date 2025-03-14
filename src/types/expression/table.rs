use luau_lexer::prelude::{Literal, LuauNumber, LuauString, Token};
use std::sync::Arc;

use super::{
    Ast, ExpressionWrap, FunctionCall, GenericDeclaration, List, NormalizedName, Table, TableKey,
    TypeValue,
};

/// An enum representing different ways in which a table value can be returned from.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum TableAccessPrefix {
    /// Just a simple access.
    ///
    /// ```lua
    /// local _ = t.name
    /// ```
    Name(Token),

    /// A function call
    ///
    /// ```lua
    /// local t = fn()
    /// ```
    FunctionCall(Arc<FunctionCall>),

    /// Accessing a table from `(...)`.
    ///
    /// ```lua
    /// local _ = ({ a = "Hello, World!" })
    /// local _ = (t)
    /// ```
    ExpressionWrap(Arc<ExpressionWrap>),
}

/// Represents an access to a table index.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct TableAccess {
    /// The actual table being indexed
    pub prefix: TableAccessPrefix,

    /// All keys accessed by the expression.
    ///
    /// ```lua
    /// local _ = t.a.b.c
    /// ```
    ///
    /// Will be `a`, `b`, `c` in this case.
    pub accessed_keys: Vec<TableAccessKey>,
}

/// Enum representing different ways in which a table's index can be accessed.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum TableAccessKey {
    /// An expression, this'll only have the enum [`TableKey::Expression`].
    Expression(Box<TableKey>),

    /// A simple name.
    Name {
        /// The `.` **before** the key.
        dot: Box<Token>,

        /// The actual key being accessed.
        name: Box<Token>,
    },
}
