//! # Type Definition
//!
//! Module containg definition for type definitions.

use std::sync::Arc;

use super::{FunctionCall, List, NormalizedName, SingleToken, Table, Var};
use crate::prelude::Expression;

/// Possible values for a type.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
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
    /// [`tuple`](TypeValue::Tuple) is that this item always have one type and only one
    /// type in it, while a [`tuple`](TypeValue::Tuple) can have any, even 0.
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
        parameters: List<NormalizedName>,

        /// The `)` character at the end of parameters and before returns
        closing_parenthesis: SingleToken,

        /// The `->` character.
        arrow: SingleToken,

        /// The return type of this function
        return_type: Arc<TypeValue>,
    },

    /// A reference to a generic type.
    ///
    /// ```lua
    /// type EmptySignal = Signal<string, ()>
    /// ```
    ///
    /// The [`generic`](TypeValue::Generic) here is `Signal`.
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
        module: SingleToken,

        /// The `.` between the module name and the type.
        dot: SingleToken,

        /// The actual name of the type being accessed.
        type_info: SingleToken,
        //TODO: Generics
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
    Table(Table),

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
    /// The difference between this and a [`variadic pack`](TypeValue::VariadicPack) is that
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
    /// See [`variadic type`](TypeValue::Variadic) to learn the difference between them.
    VariadicPack {
        /// The `...` characters.
        ellipsis: SingleToken,

        /// The name
        name: SingleToken,
    },
}

/// A struct for a type definition. Holds needed data to be able to write it back as valid
/// luau.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct TypeDefinition {
    /// The `export` keyword.
    pub export_keyword: Option<SingleToken>,

    /// The `type` keyword.
    pub type_keyword: Option<SingleToken>,

    /// The generics for this type.
    pub generics: Option<GenericDeclaration>,

    /// The name of the type.
    pub type_name: SingleToken,

    /// The `=` sign between the name and the actual value of the type.
    /// This will be `None` if this isn't it's own statement but rather
    /// in another place like parameter's type or a variable's type.
    pub equal_sign: Option<SingleToken>,

    /// The [`actual definition`](TypeValue) of the type.
    pub type_value: Arc<TypeValue>,
}

/// A generic declaration parameter used in [`generics declarations`](GenericDeclaration).
/// Can either be a name or a variadic pack.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
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
/// A generic declaration parameter used in [`generic declarations`](GenericDeclaration).
/// Consists of a [`parameter info`](GenericParameterInfo) and an optional default type.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct GenericDeclarationParameter {
    /// The parameter passed as a generic type, can be a simple name or a generic pack.
    pub parameter: GenericParameterInfo,

    /// The default type.
    pub default: Option<GenericParameterInfoDefault>,
}

/// Struct holding **default** values for generic arguments.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum GenericParameterInfoDefault {
    /// A simple name.
    ///
    /// ```lua
    /// type Foo<T = string> = "Foo"
    /// ```
    Name {
        /// The `=` character.
        equal_sign: SingleToken,

        /// The name of the type.
        name: SingleToken,
    },

    /// A generic pack.
    ///
    /// ```lua
    /// type Foo<T... = string...> = "Foo"
    /// type Bar<T... = ...string> = "Bar"
    /// type Qux<T... = (string, number)> = "Qux"
    /// ```
    Pack {
        /// The `=` character.
        equal_sign: SingleToken,

        /// The type itself..
        r#type: TypeValue,
    },
}

/// The generics used in a [`type definition`](TypeDefinition).
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct GenericDeclaration {
    /// The `<` character.
    pub right_arrow: SingleToken,

    /// The actual generics.
    pub generics: List<GenericDeclarationParameter>,

    /// The `>` character.
    pub left_arrow: SingleToken,
}

/// Possible errors converting from an expression to a type definition.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum ConversionError {
    /// Function calls can't be converted to types since the parser won't look for the
    /// variable and thus can't get it's return type.
    FunctionCall(FunctionCall),

    /// Variables calls can't be converted to types since the parser won't look for them.
    Var(Var),

    /// Unary expressions require metatables. If you wish to just use the value of the
    /// expression, pass the inner one and not the unary.
    UnaryExpression {
        /// The operator.
        operator: SingleToken,

        /// The actual expression this operator is affecting.
        expression: Arc<Expression>,
    },

    /// Binary expressions require metatables. If you wish to just use the value of the
    /// expression, pass the inner one and not the binary.
    BinaryExpression {
        /// The left expression.
        left: Arc<Expression>,

        /// The operator between the expressions.
        operator: SingleToken,

        /// The right expression.
        right: Arc<Expression>,
    },
}
