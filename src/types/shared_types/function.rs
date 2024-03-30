//! Local and global functions.

use super::{Ast, FunctionParameter, List, SingleToken, TableKey, TypeValue};

/// A struct representing a local function.
#[derive(Clone, Debug)]
pub struct LocalFunction {
    /// The `local` keyword.
    pub local_keyword: SingleToken,

    /// The `function` keyword.
    pub function_keyword: SingleToken,

    /// The name of the function.
    pub function_name: SingleToken,

    /// The `(` character.
    pub opening_parenthesis: SingleToken,

    /// The parameters that this function accepts.
    pub parameters: List<FunctionParameter>,

    /// The `)` character.
    pub closing_parenthesis: SingleToken,

    /// The return type of the function
    pub returns: TypeValue,

    /// The body of the function.
    pub body: Ast,

    /// The `end` keyword.
    pub end_keyword: SingleToken,
}

#[derive(Clone, Debug)]
pub enum GlobalFunctionName {
    SimpleName(SingleToken),

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
        table: SingleToken,

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
        keys: List<SingleToken>,

        /// The final name of the function, if it exists.
        ///
        /// ```lua
        /// local foo = {}
        ///
        /// function foo.bar.qux:Test()
        /// end
        /// ```
        ///
        /// Here, the method is `Some(Test)`. While here:
        ///
        /// ```lua
        /// local foo = {}
        ///
        /// function foo.bar.qux()
        /// end
        /// ```
        ///
        /// The method is `None` as there's no `:`.
        method: Option<SingleToken>,
    },
}

/// A struct representing a local function.
#[derive(Clone, Debug)]
pub struct GlobalFunction {
    /// The `function` keyword.
    pub function_keyword: SingleToken,

    /// The name of the function.
    pub function_name: GlobalFunctionName,

    /// The `(` character.
    pub opening_parenthesis: SingleToken,

    /// The parameters that this function accepts.
    pub parameters: List<FunctionParameter>,

    /// The `)` character.
    pub closing_parenthesis: SingleToken,

    /// The return type of the function
    pub returns: TypeValue,

    /// The body of the function.
    pub body: Ast,

    /// The `end` keyword.
    pub end_keyword: SingleToken,
}
