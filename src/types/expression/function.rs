use luau_lexer::prelude::{Literal, LuauNumber, LuauString, Token};
use std::sync::Arc;

use super::{
    Cst, Expression, GenericDeclaration, List, NormalizedName, PrefixExp, Table, TableKey,
    TypeValue,
};

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
    String(LuauString),

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
    List {
        /// The `(` character.
        open_parenthesis: Token,

        /// List of arguments passed to the function.
        arguments: List<Arc<Expression>>,

        /// The `)` character.
        close_parenthesis: Token,
    },
}
