//! Holding all needed items for functions.

use std::sync::Arc;

use crate::prelude::{SingleToken, TypeDefinition};

use super::TableKey;

/// A single parameter that a function accepts.
#[derive(Clone, Debug)]
pub struct FunctionParameter {
    /// The name of the parameter.
    pub name: SingleToken,

    /// The _[type](TypeDefinition)_ of the parameter.
    pub r#type: Option<Arc<TypeDefinition>>,
}

/// A struct representing a function name.
#[derive(Clone, Debug)]
pub enum FunctionName {
    /// An anonymous function with no name, this is used for type definitions, variables
    /// set using `=`, or functions passed to functions.
    ///
    /// ```lua
    /// type Add = (n: number, addFunction: (addTo: number) -> number) -> number
    ///
    /// local add: Add = function(n, addFunction)
    ///     return addFunction(n) + 10
    /// end
    ///
    /// add(1, function(addTo: number)
    ///     return addTo + 5
    /// end)
    /// ```
    Anonymous,

    /// A function with a single name only, this is only chosen when a function is local.
    ///
    /// ```lua
    /// local function foo()
    /// end
    /// ```
    Name(SingleToken),

    /// A function inside a table declared outside it, as a global.
    ///
    /// ```lua
    /// local t = {}
    ///
    /// function t.foo()
    /// end
    ///
    /// function t:Qux()
    /// end
    /// ```
    ///
    /// This won't count in `TableAcess`:
    ///
    /// ```lua
    /// local t = {
    ///     foo = function()
    ///     end
    /// }
    /// ```
    ///
    /// As they are defined inside the table.
    ///
    /// # Note
    ///
    /// This may also have the same result as _[a simple name](FunctionName::Name)_:
    ///
    /// ```lua
    /// function t()
    /// end
    /// ```
    ///
    /// Even though it's one name only, it's still a "global" which is why it's in
    /// _[TableAccess](FunctionName::TableAccess)_.
    TableAccess {
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
        keys: Vec<TableKey>,

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
