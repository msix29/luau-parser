//! # Type Definition
//!
//! Module containg definition for type definitions.
//!

use std::sync::Arc;

use crate::prelude::Expression;

use super::{FunctionParameter, List, Location, SingleToken, TableValue};

/// Possible values for a type.
#[derive(Clone, Debug)]
pub enum TypeValue {
    /// Just a reference to another type.
    ///
    /// ```lua
    /// type Foo = Bar
    /// ```
    Basic(SingleToken),

    /// A singletone string.
    ///
    /// ```lua
    /// type Foo = "Bar"
    /// ```
    String(SingleToken),

    /// A boolean value
    ///
    /// ```lua
    /// type Foo = true
    /// type Bar = false
    /// ```
    Boolean(SingleToken),

    /// A wrape of another type, the difference between this and a
    /// _[tuple](TypeValue::Tuple)_ is that this item always have one type and only one
    /// type in it, while a _[tuple](TypeValue::Tuple)_ can have any, even 0.
    ///
    /// ```lua
    /// type Foo = (bar)
    /// ```
    Wrap {
        /// The `(` character.
        opening_parenthesis: SingleToken,

        /// The type wrapped in between the parenthesis.
        r#type: Arc<TypeValue>,

        /// The `)` character.
        closing_parenthesis: SingleToken,
    },

    /// A function type.
    ///
    /// ```lua
    /// type Foo = (arg1: number) -> (boolean, string)`
    /// ```
    Function {
        /// Optional generics provided for the function.
        ///
        /// ```lua
        /// type Foo = <P, R>(paramter: P) -> R
        /// ```
        generics: Option<GenericDeclaration>,

        /// The `(` character at the start of the function.
        opening_parenthesis: SingleToken,

        /// The parameters this function accepts.
        parameters: List<FunctionParameter>,

        /// The `)` character at the end of parameters and before returns
        closing_parenthesis: SingleToken,

        /// The `->` character.
        arrow: SingleToken,

        /// The return type of this function
        return_type: Arc<TypeValue>,
    },

    /// A generic type.
    ///
    /// ```lua
    /// type EmptySignal = Signal<string, ()>
    /// ```
    Generic {
        /// The name of the type that has the generics.
        base: SingleToken,

        /// The `<` character.
        right_arrows: SingleToken,

        /// The actual generics.
        generics: List<TypeValue>,

        /// The `>` character.
        left_arrows: SingleToken,
    },

    /// A generic pack.
    ///
    /// ```lua
    /// <T...>
    /// ```
    GenericPack {
        /// The name.
        name: SingleToken,

        /// The `...` characters.
        ellipsis: SingleToken,
    },

    /// An intersection between two types.
    ///
    /// ```lua
    /// type Foo = Bar & Qux
    /// ```
    Intersection {
        /// The type at the start.
        left: Arc<TypeValue>,

        /// The `&` character.
        ampersand: SingleToken,

        /// The type at the end.
        right: Arc<TypeValue>,
    },

    /// An union between two types.
    ///
    /// ```lua
    /// type Foo = Bar & Qux
    /// ```
    Union {
        /// The type at the start.
        left: Arc<TypeValue>,

        /// The `|` character.
        pipe: SingleToken,

        /// The type at the end.
        right: Arc<TypeValue>,
    },

    /// An access to an exported type from a module.
    Module {
        /// the name of the module.
        module: String,

        /// The `.` between the module name and the type.
        dot: SingleToken,

        /// The actual type being accessed.
        type_info: String,
    },

    /// An optional type.
    ///
    /// ```lua
    /// type Foo = Bar?
    /// ```
    ///
    /// This is just equivalent to:
    ///
    /// ```lua
    /// type Foo = Bar | nil
    /// ```
    Optional {
        /// The actual type.
        base: Arc<TypeValue>,

        /// The `?` character.
        question_mark: SingleToken,
    },

    /// A table type.
    ///
    /// ```lua
    /// type Foo = { string }
    /// type Bar = { Qux: Foo }
    /// ```
    Table(TableValue),

    /// A `typeof` expression.
    Typeof {
        /// The `typeof` word.
        typeof_token: SingleToken,

        /// The `(` character.
        opening_parenthesis: SingleToken,

        /// The expression passed to `typeof`.
        inner: Arc<Expression>,

        /// The `)` character.
        closing_parenthesis: SingleToken,
    },

    /// A tuple of types
    ///
    /// ```lua
    /// type Foo = () -> (string, number)
    /// ```
    ///
    /// The tuple here is the return type `(string, number)`. In luau, tuples can't be
    /// their own type, meaning, this is a syntax error:
    ///
    /// ```lua
    /// type Foo = (string, number)
    /// ```
    Tuple {
        /// The `(` character.
        opening_parenthesis: SingleToken,

        /// The list of types between the parenthesis.
        types: List<TypeValue>,

        /// The `)` character.
        closing_parenthesis: SingleToken,
    },

    /// A variadic type.
    ///
    /// ```lua
    /// ...Foo
    /// ```
    ///
    /// The difference between this and a _[variadic pack](TypeValue::VariadicPack)_ is that
    /// this one can be with a type and not just a name:
    ///
    /// ```lua
    /// ...{ Foo: "Bar" }
    /// ```
    ///
    /// And is that variadic types are used in function paramterers and returns, while
    /// variadic packs are used for generics.
    Variadic {
        /// The `...` characters.
        ellipsis: SingleToken,

        /// The actual type.
        type_info: Arc<TypeValue>,
    },

    /// A variadic pack.
    ///
    /// ```lua
    /// ...Foo
    /// ```
    ///
    /// ## Note
    ///
    /// See _[variadic type](TypeValue::Variadic)_ to learn the difference between them.
    VariadicPack {
        /// The `...` characters.
        ellipsis: SingleToken,

        /// The name
        name: SingleToken,
    },
}

/// A struct for a type definition. Holds needed data to be able to write it back as valid
/// luau.
#[derive(Clone, Debug)]
pub struct TypeDefinition {
    /// The `export` keyword.
    pub export_keyword: Option<SingleToken>,

    /// The `type` keyword.
    pub type_keyword: Option<SingleToken>,

    /// The name of the type. Will always be an empty string if this is a type with no
    /// prior definition, like:
    ///
    /// ```lua
    /// local foo: { number }
    ///
    /// local function bar(qux: () -> ())
    /// end
    /// ```
    ///
    /// In both cases (`{ number }`, and `() -> ()`), they have types with no names.
    pub type_name: String,

    /// Exact location of the type name.
    pub name_location: Option<Location>,

    /// The `=` sign between the name and the actual value of the type.
    pub equal_sign: Option<SingleToken>,

    /// The _[actual definition](TypeValue)_ of the type.
    pub type_value: Arc<TypeValue>,
}


/// A generic declaration parameter used in _[generics declarations](GenericDeclaration)_.
/// Can either be a name or a variadic pack.
#[derive(Clone, Debug)]
pub enum GenericParameterInfo {
    /// A simple name, such as `T`.
    Name(SingleToken),

    /// A variadic type pack: `T...`.
    Pack {
        /// The name of the type.
        name: SingleToken,
        /// The `...` characters.
        ellipsis: SingleToken,
    },
}
/// A generic declaration parameter used in _[generic declarations](GenericDeclaration)_.
/// Consists of a _[parameter info](GenericParameterInfo)_ and an optional default type.
#[derive(Clone, Debug)]
pub struct GenericDeclarationParameter {
    pub parameter: GenericParameterInfo,
    pub default: Option<(SingleToken, TypeValue)>,
}

/// The generics used in a _[type definition](TypeDefinition)_.
#[derive(Clone, Debug)]
pub struct GenericDeclaration {
    /// The `<` character.
    pub right_arrow: SingleToken,

    /// The actual generics.
    pub generics: List<GenericDeclarationParameter>,

    /// The `>` character.
    pub left_arrow: SingleToken,
}
