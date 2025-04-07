//! Module holding if statements and all related items.

use luau_lexer::prelude::Token;
use luau_parser_derive::{Print, Range};

use crate::types::{Block, Expression, Pointer};

/// A struct representing an `if` statement
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Range, Print)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct IfStatement {
    /// The `if` keyword.
    pub if_keyword: Token,

    /// The condition for this statement.
    pub condition: Pointer<Expression>,

    /// The `then` keyword.
    pub then_keyword: Token,

    /// The body that'll be executed if condition is truthy.
    pub body: Block,

    /// Elseif branches.
    pub else_if_statements: Vec<ElseIfStatement>,

    /// The `else` part of the if statement
    pub else_statement: Option<ElseStatement>,

    /// The `end` keyword
    pub end_keyword: Token,
}

/// A struct representing an `elseif` statement
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Range, Print)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct ElseIfStatement {
    /// The `elseif` keyword.
    pub elseif_keyword: Token,

    /// The condition for this statement.
    pub condition: Pointer<Expression>,

    /// The `then` keyword.
    pub then_keyword: Token,

    /// The body that'll be executed if condition is truthy.
    pub body: Block,
}

/// A struct representing an `else` statement
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Range, Print)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct ElseStatement {
    /// The `else` keyword.
    pub else_keyword: Token,

    /// The body of the else statement.
    pub body: Block,
}
