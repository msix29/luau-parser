use luau_lexer::prelude::Token;
use luau_parser_derive::{Print, Range};

use crate::types::{
    Attribute, Block, BracketedList, Expression, GenericDeclaration, Parameter, Pointer, PrefixExp, Table, TypeValue
};

/// Different ways a function can be called.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Range, Print)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum FunctionCallInvoked {
    /// A standalone function call or one in a table.
    /// ```lua
    /// local _ = foo()
    /// local _ = t.bar()
    /// ```
    Function(Pointer<PrefixExp>),

    /// A **method** in a function, a method is a function that's called with `:` instead
    /// of `.`.
    ///
    /// ```lua
    /// local _ = t:foo()
    /// ```
    TableMethod {
        /// The table this function is from.
        table: Pointer<PrefixExp>,

        /// The colon between the table and the method name.
        colon: Pointer<Token>,

        /// The actual name of the method being called.
        method: Pointer<Token>,
    },
}

/// A struct representing a function call.
///
/// ```lua
/// local _ = foo(1, 2, 3)
/// ```
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Range, Print)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct FunctionCall {
    /// The function being called.
    pub invoked: FunctionCallInvoked,

    /// The arguments passed to the function.
    pub arguments: FunctionArguments,
}

/// All possible arguments that can be passed to a function.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Range, Print)]
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
    List(BracketedList<Pointer<FunctionArgument>>),
}

/// Arguments that can be passed in a [`FunctionCall`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Range, Print)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum FunctionArgument {
    /// A normal [`expression`](Expression).
    Expression(Pointer<Expression>),

    /// A variadic value
    ///
    /// ```lua
    /// fn(...)
    /// ```
    VariadicValues(Token),
}

/// All possible arguments that can be passed to a function.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Range, Print)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Closure {
    /// Attributes before the function.
    pub attributes: Vec<Attribute>,

    /// The `function` keyword at the start
    pub function_keyword: Token,

    /// The generics of this function.
    pub generics: Option<Pointer<GenericDeclaration>>,

    /// All [`parameters`](Name) of the function.
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
