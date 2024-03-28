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

use crate::prelude::Ast;

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
