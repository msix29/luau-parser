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
mod list;

pub use expression::*;
pub use location::*;
pub use name::*;
pub use position::*;
pub use token::*;
pub use type_definition::*;
pub use value::*;
pub use variable_declaration::*;
pub use list::*;

use std::{fmt::Display, sync::Arc};
use tree_sitter::{Node, TreeCursor};

/// A trait for a token that can be represented in a more abstract form for the user to see,
/// without maintaing original styling.
pub trait HasRawValue: Display {
    /// Get the lossy _raw value_ of this token. For lossless, see _[print](Print)_.
    fn get_raw_value(&self) -> String;
}

/// A trait to print the token as-is, while preserving all user spaces and styling.
pub trait Print: Display {
    /// Prints the whole token including all surrounding spaces.
    fn print(&self) -> String;

    /// Prints the token with only trailing spaces.
    fn print_trailing(&self) -> String;

    /// Prints the token with only leading spaces.
    fn print_leading(&self) -> String;
}

/// A trait to tell Rust that this item is an `AstNode`.
pub trait AstNode: HasRawValue /* + Print */ + Sized {
    /// Try creating this _[ast node](AstNode)_ from a _[treesitter node](Node)_. This
    /// returns a `Vec<Self>` instead of `Self` as
    /// _[variable declarations](VariableDeclaration)_ can be chained like:
    ///
    /// ```lua
    /// local foo, bar, qux
    /// ```
    ///
    /// For all other tokens, you are guaranteed that `vec.len() == 1`.
    fn try_from_node<'a>(
        node: Node<'a>,
        cursor: &mut TreeCursor<'a>,
        code_bytes: &[u8],
    ) -> Option<Vec<Self>>;
}

/// A trait for letting the compiler know that this _[ast node](AstNode)_ has a location
/// that the user can interact with. Nodes creating new scapes like if statements don't
/// have a location, while a function does as it's treated just like a variable.
pub trait HasLocation: AstNode {
    fn get_location(&self) -> Location;
}

/// All possible tokens in an _[ast](Ast)_.
#[derive(Clone, Debug)]
pub enum Token {
    /// A variable declaration.
    ///
    /// ```lua
    /// local foo = bar
    /// local bar = function()
    /// end
    /// local qux = {}
    /// ```
    VariableDeclaration(VariableDeclaration),

    /// A type definition.
    ///
    /// ```lua
    /// type Foo = Bar<string, number>
    /// export type Bar<P, R> = (param: P) -> R
    /// type qux = {}
    /// ```
    TypeDefinition(TypeDefinition),
}

/// A struct representing a scope in a file. This ast is lossless, meaning it can be
/// printed back to the code it was created from without losing any details.
#[derive(Clone, Debug, Default)]
pub struct Ast {
    /// The path pointing to the file that this _[ast](Ast)_ represents, if any.
    pub uri: Option<String>,

    /// The tokens in the **main scope** of this file.
    pub tokens: Arc<Vec<Token>>,
}

unsafe impl Send for Ast {}
unsafe impl Sync for Ast {}
