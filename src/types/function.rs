//! Local and global functions.

use super::{Ast, GenericDeclaration, List, NormalizedName, SingleToken, TypeValue};

/// A struct representing a local function.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct LocalFunction {
    /// The `local` keyword.
    pub local_keyword: SingleToken,

    /// The `function` keyword.
    pub function_keyword: SingleToken,

    /// The name of the function.
    pub function_name: SingleToken,

    /// The generics of the function.
    pub generics: Option<GenericDeclaration>,

    /// The `(` character.
    pub opening_parenthesis: SingleToken,

    /// The parameters that this function accepts.
    pub parameters: List<NormalizedName>,

    /// The `)` character.
    pub closing_parenthesis: SingleToken,

    /// The return type of the function
    pub returns: Option<TypeValue>,

    /// The body of the function.
    pub body: Ast,

    /// The `end` keyword.
    pub end_keyword: SingleToken,
}

/// An enum representing possible ways in which a global function's name can be.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum GlobalFunctionName {
    /// Just a simple name, this is usually in local functions but some people don't do so.
    SimpleName(SingleToken),

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
        ///
        /// # Note
        ///
        /// All [`ListItem`]s here will be will be [`NonTrailing`]. Amd `key.0` will always
        /// be the dot character instead.
        ///
        /// [`ListItem`]: crate::types::ListItem
        /// [`NonTrailing`]: crate::types::ListItem::NonTrailing
        keys: List<(SingleToken, SingleToken)>,

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
        method: Option<(SingleToken, SingleToken)>,
    },
}

/// A struct representing a local function.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct GlobalFunction {
    /// The `function` keyword.
    pub function_keyword: SingleToken,

    /// The name of the function.
    pub function_name: GlobalFunctionName,

    /// The generics of the function.
    pub generics: Option<GenericDeclaration>,

    /// The `(` character.
    pub opening_parenthesis: SingleToken,

    /// The parameters that this function accepts.
    pub parameters: List<NormalizedName>,

    /// The `)` character.
    pub closing_parenthesis: SingleToken,

    /// The return type of the function
    pub returns: Option<TypeValue>,

    /// The body of the function.
    pub body: Ast,

    /// The `end` keyword.
    pub end_keyword: SingleToken,
}
