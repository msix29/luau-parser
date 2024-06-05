//! Module holding if statements and all related items.

use std::sync::Arc;

use crate::{
    generate_derives,
    prelude::{Ast, Expression, Token},
};

generate_derives! {
    /// A struct representing an `if` statement
    pub struct IfStatement {
    /// The `if` keyword.
    pub if_keyword: Token,

    /// The condition for this statement.
    pub condition: Arc<Expression>,

    /// The `then` keyword.
    pub then_keyword: Token,

    /// The body that'll be excuted if condition is truthy.
    pub body: Ast,

    /// Elseif branches.
    pub else_if_statements: Vec<ElseIfStatement>,

    /// The `else` part of the if statement
    pub else_statement: Option<ElseStatement>,

    /// The `end` keyword
    pub end_keyword: Token,
    }
}

generate_derives! {
    /// A struct representing an `elseif` statement
    pub struct ElseIfStatement {
    /// The `elseif` keyword.
    pub elseif_keyword: Token,

    /// The condition for this statement.
    pub condition: Arc<Expression>,

    /// The `then` keyword.
    pub then_keyword: Token,

    /// The body that'll be excuted if condition is truthy.
    pub body: Ast,
    }
}

generate_derives! {
    /// A struct representing an `else` statement
    pub struct ElseStatement {
    /// The `else` keyword.
    pub else_keyword: Token,

    /// The body of the else statement.
    pub body: Ast,
    }
}
