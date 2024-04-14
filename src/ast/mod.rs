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
pub mod range;
pub mod name;
pub mod position;
pub mod set_expressions;
pub mod token;
pub mod type_definition;

use crate::prelude::{Ast, HasRange, Range, Print, Statement};

impl Ast {
    /// Returns the code that was behind this AST as-is, without any modifications and
    /// without losing on any details.
    pub fn print(&self) -> String {
        let len = self.tokens.len();
        if len == 0 {
            return String::new();
        }

        let mut str = String::new();
        let last_index = len - 1;
        for (i, token) in self.tokens.iter().enumerate() {
            if i == last_index {
                str.push_str(&token.print());
            } else {
                str.push_str(token.print().trim_end());
            }
        }

        str
    }
}

/// Implements a specific trait for [`Statement`]s.
macro_rules! impl_statement {
    ($trait: ident, $fn_name: ident, $return: ty) => {
        impl $trait for Statement {
            fn $fn_name(&self) -> $return {
                match self {
                    Statement::LocalAssignment(value) => value.$fn_name(),
                    Statement::TypeDefinition(value) => value.$fn_name(),
                    Statement::IfStatement(value) => value.$fn_name(),
                    Statement::DoBlock(value) => value.$fn_name(),
                    Statement::GenericFor(value) => value.$fn_name(),
                    Statement::NumericalFor(value) => value.$fn_name(),
                    Statement::RepeatBlock(value) => value.$fn_name(),
                    Statement::WhileLoop(value) => value.$fn_name(),
                    Statement::SetExpression(value) => value.$fn_name(),
                    Statement::CompoundSetExpression(value) => value.$fn_name(),
                    Statement::FunctionCall(value) => value.$fn_name(),
                    Statement::LocalFunction(value) => value.$fn_name(),
                    Statement::GlobalFunction(value) => value.$fn_name(),
                    Statement::Comment(value) => value.$fn_name(),
                }
            }
        }
    };
}

impl_statement!(HasRange, get_range, Range);
impl_statement!(Print, print, String);

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
