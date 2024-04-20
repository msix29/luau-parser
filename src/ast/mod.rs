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
pub mod name;
pub mod position;
pub mod range;
pub mod set_expressions;
pub mod statement;
pub mod token;
pub mod type_definition;

use crate::prelude::{Ast, HasRange, Print, Range, SingleToken, Statement};

impl Ast {
    /// Returns the code that was behind this AST as-is, without any modifications and
    /// without losing on any details.
    pub fn print(&self) -> String {
        let len = self.statements.len();
        if len == 0 {
            return String::new();
        }

        let mut str = String::new();
        let last_index = len - 1;
        for (i, token) in self.statements.iter().enumerate() {
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
                    Self::LocalAssignment(value) => value.$fn_name(),
                    Self::TypeDefinition(value) => value.$fn_name(),
                    Self::IfStatement(value) => value.$fn_name(),
                    Self::DoBlock(value) => value.$fn_name(),
                    Self::GenericFor(value) => value.$fn_name(),
                    Self::NumericalFor(value) => value.$fn_name(),
                    Self::RepeatBlock(value) => value.$fn_name(),
                    Self::WhileLoop(value) => value.$fn_name(),
                    Self::SetExpression(value) => value.$fn_name(),
                    Self::CompoundSetExpression(value) => value.$fn_name(),
                    Self::FunctionCall(value) => value.$fn_name(),
                    Self::LocalFunction(value) => value.$fn_name(),
                    Self::GlobalFunction(value) => value.$fn_name(),
                    Self::Comment(value) => value.$fn_name(),
                }
            }
        }
    };
}

impl_statement!(HasRange, get_range, Range);
impl_statement!(Print, print, String);

impl Statement {
    /// Get the body of this rule, if this rule doesn't start a new scope, `None` is returned.
    ///
    /// # Note
    ///
    /// This returns `None` for variables and set expressions.
    pub fn try_get_body(&self) -> Option<&Ast> {
        match self {
            Self::IfStatement(value) => Some(&value.body),
            Self::DoBlock(value) => Some(&value.body),
            Self::GenericFor(value) => Some(&value.do_block.body),
            Self::NumericalFor(value) => Some(&value.do_block.body),
            Self::RepeatBlock(value) => Some(&value.body),
            Self::WhileLoop(value) => Some(&value.do_block.body),
            Self::LocalFunction(value) => Some(&value.body),
            Self::GlobalFunction(value) => Some(&value.body),
            _ => None,
        }
    }
}

impl Print for (Statement, Option<SingleToken>) {
    fn print(&self) -> String {
        format!("{}{}", self.0.print(), self.1.as_ref().map_or("", |_| ";"))
    }
}
