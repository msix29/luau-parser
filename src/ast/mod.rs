//! # Abstract Syntax Tree
//!
//! This module is only responsible for loading in needed files which implements multiple
//! helper traits for AST-related structs.
//!

pub mod block;
pub mod expression;
pub mod list;
pub mod local_assignment;
pub mod location;
pub mod name;
pub mod position;
pub mod set_expressions;
pub mod token;
pub mod type_definition;

use crate::prelude::{Ast, HasLocation, Location, Statement};

impl Ast {
    /// Returns the code that was behind this AST as-is, without any modifications and
    /// without losing on any details.
    pub fn print(&self) -> String {
        todo!()
        // self.tokens
        //     .iter()
        //     .map(|token| match token {
        //         Token::VariableDeclaration(value) => value.print(),
        //         Token::TypeDefinition(value) => value.print(),
        //         Token::IfStatement(_) => todo!(),

        //     })
        //     .collect::<String>()
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

        }
    }
}
