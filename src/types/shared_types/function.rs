//! Local and global functions.

use super::{Ast, FunctionParameter, List, SingleToken, TypeValue};

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
