//! # Abstract Syntax Tree
//!
//! This module is only responsible for loading in needed files which implements multiple
//! helper traits for AST-related structs.
//!

pub mod block;
pub mod comment;
pub mod expression;
pub mod function;
pub mod list;
pub mod local_assignment;
pub mod location;
pub mod name;
pub mod position;
pub mod set_expressions;
pub mod token;
pub mod type_definition;

use crate::prelude::{Ast, HasLocation, Location, Print, Statement};

impl Ast {
    /// Returns the code that was behind this AST as-is, without any modifications and
    /// without losing on any details.
    pub fn print(&self) -> String {
        self.tokens
            .iter()
            .map(|token| match token {
                Statement::LocalAssignment(value) => value.print(),
                Statement::TypeDefinition(_) => todo!(),
                Statement::IfStatement(_) => todo!(),
                Statement::DoBlock(_) => todo!(),
                Statement::GenericFor(_) => todo!(),
                Statement::NumericalFor(_) => todo!(),
                Statement::RepeatBlock(_) => todo!(),
                Statement::WhileLoop(_) => todo!(),
                Statement::SetExpression(_) => todo!(),
                Statement::CompoundSetExpression(_) => todo!(),
                Statement::FunctionCall(_) => todo!(),
                Statement::LocalFunction(_) => todo!(),
                Statement::GlobalFunction(_) => todo!(),
                Statement::Comment(_) => todo!(),
            })
            .collect::<String>()
    }
}

impl HasLocation for Statement {
    fn get_location(&self) -> Location {
        match self {
            Statement::LocalAssignment(value) => value.get_location(),
            Statement::TypeDefinition(value) => value.get_location(),
            Statement::IfStatement(value) => value.get_location(),
            Statement::DoBlock(value) => value.get_location(),
            Statement::GenericFor(value) => value.get_location(),
            Statement::NumericalFor(value) => value.get_location(),
            Statement::RepeatBlock(value) => value.get_location(),
            Statement::WhileLoop(value) => value.get_location(),
            Statement::SetExpression(value) => value.get_location(),
            Statement::CompoundSetExpression(value) => value.get_location(),
            Statement::FunctionCall(value) => value.get_location(),
            Statement::LocalFunction(value) => value.get_location(),
            Statement::GlobalFunction(value) => value.get_location(),
            Statement::Comment(value) => value.get_location(),
        }
    }
}
impl Statement {
    /// Get th body of this rule, if this rule doesn't start a new scope, `None` is returned.
    ///
    /// # Note
    ///
    /// This returns `None` for variables and set expressions.
    pub fn try_get_body(&self) -> Option<&Ast> {
        match self {
            Statement::IfStatement(value) => Some(&value.body),
            Statement::DoBlock(value) => Some(&value.body),
            Statement::GenericFor(value) => Some(&value.do_block.body),
            Statement::NumericalFor(value) => Some(&value.do_block.body),
            Statement::RepeatBlock(value) => Some(&value.body),
            Statement::WhileLoop(value) => Some(&value.do_block.body),
            Statement::LocalFunction(value) => Some(&value.body),
            Statement::GlobalFunction(value) => Some(&value.body),
            _ => None,
        }
    }
}
