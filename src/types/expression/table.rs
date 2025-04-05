use luau_lexer::prelude::Token;
use luau_parser_derive::{Print, Range};

use crate::types::{ExpressionWrap, FunctionCall, Pointer, TableKey};

/// An enum representing different ways in which a table value can be returned from.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Range, Print)]
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
    FunctionCall(Pointer<FunctionCall>),

    /// Accessing a table from `(...)`.
    ///
    /// ```lua
    /// local _ = ({ a = "Hello, World!" })
    /// local _ = (t)
    /// ```
    ExpressionWrap(Pointer<ExpressionWrap>),
}

/// Represents an access to a table index.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Range, Print)]
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
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Range, Print)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum TableAccessKey {
    /// An expression, this'll only have the enum [`TableKey::Expression`].
    Expression(Pointer<TableKey>),

    /// A simple name.
    Name {
        /// The `.` **before** the key.
        dot: Pointer<Token>,

        /// The actual key being accessed.
        name: Pointer<Token>,
    },
}
