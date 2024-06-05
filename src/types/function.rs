//! Local and global functions.

use std::sync::Arc;

use super::{Ast, GenericDeclaration, List, NormalizedName, Token, TypeValue};
use crate::generate_derives;

generate_derives! {
    /// A struct representing a local function.
    pub struct LocalFunction {
        /// The `local` keyword.
        pub local_keyword: Token,

        /// The `function` keyword.
        pub function_keyword: Token,

        /// The name of the function.
        pub function_name: Token,

        /// The generics of the function.
        pub generics: Option<GenericDeclaration>,

        /// The `(` character.
        pub opening_parenthesis: Token,

        /// The parameters that this function accepts.
        pub parameters: List<NormalizedName>,

        /// The `)` character.
        pub closing_parenthesis: Token,

        /// The `:` character between closing parenthesis and returns.
        pub colon: Option<Token>,

        /// The return type of the function
        pub returns: Option<Arc<TypeValue>>,

        /// The body of the function.
        pub body: Ast,

        /// The `end` keyword.
        pub end_keyword: Token,
    }
}

generate_derives! {
    /// An enum representing possible ways in which a global function's name can be.
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
            /// All [`ListItem`]s here will be will be [`NonTrailing`]. Amd `key.0` will always
            /// be the dot character instead.
            ///
            /// [`ListItem`]: crate::types::ListItem
            /// [`NonTrailing`]: crate::types::ListItem::NonTrailing
            keys: List<(Token, Token)>,

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
            method: Option<(Token, Token)>,
        },
    }
}

generate_derives! {
    /// A struct representing a local function.
    pub struct GlobalFunction {
        /// The `function` keyword.
        pub function_keyword: Token,

        /// The name of the function.
        pub function_name: GlobalFunctionName,

        /// The generics of the function.
        pub generics: Option<GenericDeclaration>,

        /// The `(` character.
        pub opening_parenthesis: Token,

        /// The parameters that this function accepts.
        pub parameters: List<NormalizedName>,

        /// The `)` character.
        pub closing_parenthesis: Token,

        /// The `:` character between closing parenthesis and returns.
        pub colon: Option<Token>,

        /// The return type of the function
        pub returns: Option<Arc<TypeValue>>,

        /// The body of the function.
        pub body: Ast,

        /// The `end` keyword.
        pub end_keyword: Token,
    }
}
