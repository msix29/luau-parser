//! Module holding if statements and all related items.

use std::sync::Arc;

use crate::prelude::{Ast, Expression, SingleToken};

/// A struct representing an if statement
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct IfStatement {
    /// The `if` keyword.
    pub if_keyword: SingleToken,

    /// The condition for this statement.
    pub condition: Arc<Expression>,

    /// The `then` keyword.
    pub then_keyword: SingleToken,

    /// The body that'll be excuted if condition is truthy.
    pub body: Ast,

    /// Elseif branches.
    pub else_if_expressions: Vec<ElseIfStatement>,

    /// The `else` part of the if statement
    pub else_expression: Option<ElseStatement>,

    /// The `end` keyword
    pub end_keyword: SingleToken,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
/// A struct representing an if statement
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct ElseIfStatement {
    /// The `elseif` keyword.
    pub elseif_keyword: SingleToken,

    /// The condition for this statement.
    pub condition: Arc<Expression>,

    /// The `then` keyword.
    pub then_keyword: SingleToken,

    /// The body that'll be excuted if condition is truthy.
    pub body: Ast,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
/// A struct representing an if statement
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct ElseStatement {
    /// The `else` keyword.
    pub else_keyword: SingleToken,

    /// The body of the else statement.
    pub body: Ast,
}
