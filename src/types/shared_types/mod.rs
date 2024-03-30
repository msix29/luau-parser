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

mod block;
mod expression;
mod list;
mod local_assignment;
mod location;
mod name;
mod position;
mod set_expressions;
mod token;
mod type_definition;
mod value;

pub use block::*;
pub use expression::*;
pub use list::*;
pub use local_assignment::*;
pub use location::*;
pub use name::*;
pub use position::*;
pub use set_expressions::*;
pub use token::*;
pub use type_definition::*;
pub use value::*;

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

/// A trait to tell Rust that this item is a `LuauStatement`.
pub trait LuauStatement: Sized {
    /// Try creating this _[statement](LuauStatement)_ from a _[treesitter node](Node)_.
    fn try_from_node<'a>(
        node: Node<'a>,
        cursor: &mut TreeCursor<'a>,
        code_bytes: &[u8],
    ) -> Option<Self>;
}

/// A trait for letting the compiler know that this specifc item has a location.
pub trait HasLocation {
    /// Get the location of the node.
    fn get_location(&self) -> Location;
}

/// A trait for letting the compiler know that this specific item may have a location.
pub trait MightHaveLocation {
    /// Try getting the location of the node.
    fn try_get_location(&self) -> Option<Location>;
}

/// All possible tokens in an _[ast](Ast)_.
#[derive(Clone, Debug)]
pub enum Statement {
    /// A variable declaration.
    ///
    /// ```lua
    /// local foo = bar
    /// local bar = function()
    /// end
    /// local qux = {}
    /// ```
    LocalAssignment(LocalAssignment),

    /// A type definition.
    ///
    /// ```lua
    /// type Foo = Bar<string, number>
    /// export type Bar<P, R> = (param: P) -> R
    /// type qux = {}
    /// ```
    TypeDefinition(TypeDefinition),

    /// An if statement.
    ///
    /// ```lua
    /// if a then
    ///     print("It's a")
    /// elseif b then
    ///     print("It's b")
    /// else
    ///     print("It's neither a or b :(")
    /// end
    /// ```
    IfStatement(IfStatement),

    /// A do block.
    ///
    /// ```lua
    /// do
    ///     print("Hello, World!")
    /// end
    /// ```
    ///
    /// # Note
    ///
    /// This struct isn't used for while or for loops, they have their own tokens, and have
    /// do blocks as part of their token.
    DoBlock(DoBlock),

    /// A generic for loop.
    ///
    /// ```lua
    /// for i, v in ipairs(t) do
    ///     print(`{i}: {v}`)
    /// end
    /// ```
    GenericFor(GenericFor),

    /// A numerical for loop.
    ///
    /// ```lua
    /// for i = 1, 100, 2 do
    ///     print(i)
    /// end
    /// ```
    NumericalFor(NumericalFor),

    /// A repeat block.
    ///
    /// ```lua
    /// local i = 0
    /// repeat
    ///     print(i)
    ///     i += 1
    /// until i == 10
    /// ```
    RepeatBlock(RepeatBlock),

    /// A while loop.
    ///
    /// ```lua
    /// local i = 0
    /// while i <= 10 do
    ///     print(i)
    ///     i += 1
    /// end
    /// ```
    WhileLoop(WhileLoop),

    /// A set expression.
    ///
    /// ```lua
    /// a = "test"
    /// b, c = true, false, 1
    /// d, e, f = foo()
    /// ```
    SetExpression(SetExpression),

    /// A compound set expression.
    ///
    /// ```lua
    /// foo += 1
    /// bar //= 2
    /// ```
    CompoundSetExpression(CompoundSetExpression),

    /// A function call.
    ///
    /// ```lua
    /// local _ = foo(1, 2, 3)
    /// ```
    FunctionCall(FunctionCall),
}

/// A struct representing a scope in a file. This ast is lossless, meaning it can be
/// printed back to the code it was created from without losing any details.
#[derive(Clone, Debug, Default)]
pub struct Ast {
    /// The path pointing to the file that this _[ast](Ast)_ represents, if any.
    pub uri: Option<String>,

    /// The tokens in the **main scope** of this file.
    pub tokens: Arc<Vec<Statement>>,
}
