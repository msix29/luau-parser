//! # Abstract Syntax Tree
//!
//! This module is only responsible for loading in needed files which implements multiple
//! helper traits for AST-related structs.
//!

pub mod block;
pub mod expression;
pub mod list;
pub mod location;
pub mod name;
pub mod position;
pub mod token;
pub mod type_definition;
pub mod local_assignment;

use crate::prelude::{Ast, HasLocation, Token};

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

impl HasLocation for Token {
    fn get_location(&self) -> crate::prelude::Location {
        match self {
            Token::LocalAssignment(value) => value.get_location(),
            Token::TypeDefinition(value) => value.get_location(),
            Token::IfStatement(value) => value.get_location(),
            Token::DoBlock(value) => value.get_location(),
            Token::GenericFor(value) => value.get_location(),
            Token::NumericalFor(value) => value.get_location(),
            Token::RepeatBlock(value) => value.get_location(),
            Token::WhileLoop(value) => value.get_location(),
        }
    }
}
