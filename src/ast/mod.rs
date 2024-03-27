//! # Abstract Syntax Tree
//!
//! This module is only responsible for loading in needed files which implements multiple
//! helpful traits for AST-related structs.
//!

pub mod expression;
pub mod list;
pub mod name;
pub mod token;
pub mod type_definition;
pub mod variable_declaration;

use crate::prelude::{Ast, Print, Token};

impl Ast {
    pub fn print(&self) -> String {
        self.tokens
            .iter()
            .map(|token| match token {
                Token::VariableDeclaration(value) => value.print(),
                Token::TypeDefinition(value) => value.print(),
            })
            .collect::<String>()
    }
}
