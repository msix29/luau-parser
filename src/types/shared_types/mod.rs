//! # Shared types
//!
//! Module holding types that'll be used everywhere around the parser and most likely
//! outside it too, like in a formatter or a lsp.
//!
//! ## Note
//!
//! This file only contains the definitions for items, for actual implementations,
//! check the files under `src/ast`. Each type will have it's implementation in
//! the same location, ex. types in `shared_types/value/function.rs` will have
//! their implementations in `ast/value/function.rs`.
//!

mod expression;
mod location;
mod name;
mod position;
mod token;
mod type_definition;
mod value;
mod variable_declaration;

pub use expression::*;
pub use location::*;
pub use name::*;
pub use position::*;
pub use token::*;
pub use type_definition::*;
pub use value::*;
pub use variable_declaration::*;

use std::{fmt::Display, sync::Arc};
use tree_sitter::{Node, TreeCursor};

pub trait HasRawValue: Display {
    fn get_raw_value(&self) -> String;
}

pub trait Print: Display {
    /// Prints the whole token including all surrounding spaces.
    fn print(&self) -> String;

    /// Prints the token with only trailing spaces.
    fn print_trailing(&self) -> String;

    /// Prints the token with only leading spaces.
    fn print_leading(&self) -> String;
}

pub trait AstNode: HasRawValue + Sized {
    fn try_from_node<'a>(
        node: Node<'a>,
        cursor: &mut TreeCursor<'a>,
        code_bytes: &[u8],
    ) -> Option<Vec<Self>>;
}

pub trait HasLocation: AstNode {
    fn get_location(&self) -> Location;
}

#[derive(Clone, Debug)]
pub enum Token {
    VariableDeclaration(VariableDeclaration),
    TypeDefinition(TypeDefinition),
}

#[derive(Clone, Debug, Default)]
pub struct Ast {
    pub uri: Option<String>,
    pub tokens: Arc<Vec<Token>>,
}

unsafe impl Send for Ast {}
unsafe impl Sync for Ast {}
