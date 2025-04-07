//! Local and global functions.

use luau_lexer::prelude::Token;
use luau_parser_derive::{Print, Range};

use crate::types::{Block, BracketedList, GenericDeclaration, Pointer, TableAccessKey, TypeValue};

/// A struct representing a local function.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Range, Print)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct LocalFunction {
    /// Attributes before the function.
    pub attributes: Vec<Attribute>,

    /// The `local` keyword.
    pub local_keyword: Token,

    /// The `function` keyword.
    pub function_keyword: Token,

    /// The name of the function.
    pub function_name: Token,

    /// The generics of the function.
    pub generics: Option<Pointer<GenericDeclaration>>,

    /// The parameters that this function accepts.
    pub parameters: BracketedList<Parameter>,

    /// The `:` character between closing parenthesis and returns.
    pub colon: Option<Pointer<Token>>,

    /// The return type of the function
    pub return_type: Option<Pointer<TypeValue>>,

    /// The body of the function.
    pub body: Block,

    /// The `end` keyword.
    pub end_keyword: Token,
}

/// Parameter that a function can have. The difference between this and
/// [`Name`](crate::types::Name) is the fact that [`Parameter.name`](Parameter::name)
/// can match variadic values (`...`) while `Name` can't.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Range, Print)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Parameter {
    /// The actual name.
    pub name: Token,

    /// `:` character.
    pub colon: Option<Token>,

    /// The type that was with this name, defined with the `: type` syntax.
    #[range_or = "name"]
    pub r#type: Option<Pointer<TypeValue>>,
}

/// An attribute that can be placed before a function.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Range, Print)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Attribute {
    /// `@` character.
    pub at: Token,

    /// The actual attribute.
    pub attribute: Token,
}

/// An enum representing possible ways in which a global function's name can be.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Print)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum GlobalFunctionName {
    /// Just a simple name, this is usually in local functions but some people don't do so.
    SimpleName(Token),

    /// A table.
    ///
    /// ```lua
    /// function foo.bar()
    /// end
    /// ```
    Table {
        /// The table that's being accessed
        ///
        /// ```lua
        /// local foo = {}
        ///
        /// function foo.bar()
        /// end
        /// ```
        ///
        /// Here, the table is `foo`.
        table: Token,

        /// Fields accessed from the table.
        ///
        /// ```lua
        /// local foo = {}
        ///
        /// function foo.bar.qux:Test()
        /// end
        /// ```
        ///
        /// Here, the keys are `bar` and `qux`.
        ///
        /// # Note
        ///
        /// All [`ListItem`]s here will be will be [`NonTrailing`]. And `key.0` will always
        /// be the dot character.
        ///
        /// [`ListItem`]: crate::types::ListItem
        /// [`NonTrailing`]: crate::types::ListItem::NonTrailing
        keys: Vec<TableAccessKey>,

        /// The final name of the function, if it exists.
        ///
        /// ```lua
        /// local foo = {}
        ///
        /// function foo.bar.qux:Test()
        /// end
        /// ```
        ///
        /// Here, the method is `Some((Colon, Test))`. While here:
        ///
        /// ```lua
        /// local foo = {}
        ///
        /// function foo.bar.qux()
        /// end
        /// ```
        ///
        /// The method is `None` as there's no `:`.
        method: Option<Pointer<(Token, Token)>>,
    },
}

/// A struct representing a local function.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Range, Print)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct GlobalFunction {
    /// Attributes before the function.
    pub attributes: Vec<Attribute>,

    /// The `function` keyword.
    pub function_keyword: Token,

    /// The name of the function.
    pub function_name: GlobalFunctionName,

    /// The generics of the function.
    pub generics: Option<Pointer<GenericDeclaration>>,

    /// The parameters that this function accepts.
    pub parameters: BracketedList<Parameter>,

    /// The `:` character between closing parenthesis and returns.
    pub colon: Option<Pointer<Token>>,

    /// The return type of the function
    pub return_type: Option<Pointer<TypeValue>>,

    /// The body of the function.
    pub body: Block,

    /// The `end` keyword.
    pub end_keyword: Token,
}
