//! Module holding if statements and all related items.

use luau_lexer::prelude::Token;
use std::sync::Arc;

use crate::prelude::Expression;

use super::Block;

/// A struct representing an `if` statement
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct IfStatement {
    /// The `if` keyword.
    pub if_keyword: Token,

    /// The condition for this statement.
    pub condition: Arc<Expression>,

    /// The `then` keyword.
    pub then_keyword: Token,

    /// The body that'll be excuted if condition is truthy.
    pub body: Block,

    /// Elseif branches.
    pub else_if_statements: Vec<ElseIfStatement>,

    /// The `else` part of the if statement
    pub else_statement: Option<ElseStatement>,

    /// The `end` keyword
    pub end_keyword: Token,
}

/// A struct representing an `elseif` statement
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct ElseIfStatement {
    /// The `elseif` keyword.
    pub elseif_keyword: Token,

    /// The condition for this statement.
    pub condition: Arc<Expression>,

    /// The `then` keyword.
    pub then_keyword: Token,

    /// The body that'll be excuted if condition is truthy.
    pub body: Block,
}

/// A struct representing an `else` statement
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct ElseStatement {
    /// The `else` keyword.
    pub else_keyword: Token,

    /// The body of the else statement.
    pub body: Block,
}
