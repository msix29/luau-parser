//! Holding all needed items for functions.

use std::sync::Arc;

use crate::prelude::{Ast, List, Location, SingleToken, TypeDefinition, TypeValue};

use super::TableKey;

/// A single parameter that a function accepts.
#[derive(Clone, Debug)]
pub struct FunctionParameter {
    /// The name of the parameter.
    pub name: SingleToken,

    /// The _[type](TypeDefinition)_ of the parameter.
    pub r#type: Arc<TypeDefinition>,

    /// Whether or not this parameter is variadic.
    pub is_variadic: bool,

    /// Exact location of the parmeter
    pub location: Location,
}

/// A single return that a function has.
#[derive(Clone, Debug)]
pub struct FunctionReturn {
    /// The _[type](TypeDefinition)_ of the return.
    pub r#type: Arc<TypeDefinition>,

    /// Whether or not this return is variadic.
    pub is_variadic: bool,

    /// Exact location of the return.
    pub location: Location,
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
    }
}

/// The actual value representing a function for the _[value](crate::prelude::Value)_ enum.
#[derive(Clone, Debug)]
pub struct FunctionValue {
    /// The `local` keyword before the function (if any). `Some` in local functions only.
    pub local_keyword: Option<SingleToken>,

    /// The `function` keyword at the start (if any), only `None` in
    /// _[type definitions](TypeDefinition)_.
    pub function_keyword: Option<SingleToken>,

    /// The actual name of the function.
    pub function_name: FunctionName,

    /// All _[parameters](FunctionParameter)_ of the function.
    pub parameters: Arc<List<FunctionParameter>>,

    /// All _[returns](FunctionReturn)_ of the function
    pub returns: Arc<TypeValue>,

    /// The body of the function.
    pub body: Arc<Ast>,

    /// The `end` keyword (if any), only `None` in _[type definitions](TypeDefinition)_.
    pub end_keyword: Option<SingleToken>,
}
