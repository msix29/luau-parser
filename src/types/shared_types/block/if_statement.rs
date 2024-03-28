//! Module holding if statements and all related items.

use std::sync::Arc;

use crate::prelude::{Ast, Expression, Location, SingleToken};

#[derive(Clone, Debug)]
/// A struct representing an if statement
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

    /// the location of the full if statement.
    pub location: Location
}

#[derive(Clone, Debug)]
/// A struct representing an if statement
pub struct ElseIfStatement {
    /// The `elseif` keyword.
    pub elseif_keyword: SingleToken,

    /// The condition for this statement.
    pub condition: Arc<Expression>,

    /// The `then` keyword.
    pub then_keyword: SingleToken,

    /// The body that'll be excuted if condition is truthy.
    pub body: Ast,

    /// the location of the full elseif statement.
    pub location: Location
}

#[derive(Clone, Debug)]
/// A struct representing an if statement
pub struct ElseStatement {
    /// The `else` keyword.
    pub else_keyword: SingleToken,

    /// The body of the else statement.
    pub body: Ast,

    /// the location of the full else statement.
    pub location: Location
}
