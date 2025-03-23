use luau_lexer::prelude::Token;
use std::sync::Arc;

use super::{Expression, PrefixExp, Table};
use crate::types::{Block, BracketedList, GenericDeclaration, Name, TypeValue};

/// Different ways a function can be called.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum FunctionCallInvoked {
    /// A standalone function call or one in a table.
    /// ```lua
    /// local _ = foo()
    /// local _ = t.bar()
    /// ```
    Function(Arc<PrefixExp>),

    /// A **method** in a function, a method is a function that's called with `:` instead
    /// of `.`.
    ///
    /// ```lua
    /// local _ = t:foo()
    /// ```
    TableMethod {
        /// The table this function is from.
        table: Arc<PrefixExp>,

        /// The colon between the table and the method name.
        colon: Box<Token>,

        /// The actual name of the method being called.
        method: Box<Token>,
    },
}

/// A struct representing a function call.
///
/// ```lua
/// local _ = foo(1, 2, 3)
/// ```
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct FunctionCall {
    /// The function being called.
    pub invoked: FunctionCallInvoked,

    /// The arguments passed to the function.
    pub arguments: FunctionArguments,
}

/// All possible arguments that can be passed to a function.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum FunctionArguments {
    /// A standalone string.
    ///
    /// ```lua
    /// local _ = foo"Hello, World!"
    /// ```
    String(Token),

    /// A standalone table.
    ///
    /// ```lua
    /// local _ = foo { bar = "Hello, World!" }
    /// ```
    Table(Table),

    /// A list of arguments.
    ///
    /// ```lua
    /// local _ = foo(1, 2, 3)
    /// ```
    List(BracketedList<Arc<Expression>>),
}

/// All possible arguments that can be passed to a function.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Closure {
    /// The `function` keyword at the start
    pub function_keyword: Token,

    /// The generics of this function.
    pub generics: Option<Box<GenericDeclaration>>,

    /// All [`parameters`](Name) of the function.
    pub parameters: BracketedList<Name>,

    /// The `:` character between closing parenthesis and returns.
    pub colon: Box<Option<Token>>,

    /// The return type of the function
    pub return_type: Option<Arc<TypeValue>>,

    /// The body of the function.
    pub body: Block,

    /// The `end` keyword.
    pub end_keyword: Box<Token>,
}
