//! # Type Definition
//!
//! Module containg definition for type definitions.

use luau_lexer::prelude::Token;
use luau_parser_derive::{Print, Range};

use crate::types::{Bracketed, BracketedList, Expression, FunctionCall, Name, Pointer, Table, Var};

/// Possible values for a type.
#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord, Range, Print)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum TypeValue {
    /// This [`TypeValue`] had a syntax error.
    #[default]
    ERROR,

    /// A singletone string.
    ///
    /// ```lua
    /// type Foo = "Bar"
    /// ```
    String(Token),

    /// A boolean value
    ///
    /// ```lua
    /// type Foo = true
    /// type Bar = false
    /// ```
    Boolean(Token),

    /// `nil`
    ///
    /// ```lua
    /// type Foo = nil
    /// ```
    Nil(Token),

    /// A wrape of another type, the difference between this and a
    /// [`tuple`](TypeValue::Tuple) is that this item always have one type and only one
    /// type in it, while a [`tuple`](TypeValue::Tuple) can have any, even 0.
    ///
    /// ```lua
    /// type Foo = (bar)
    /// ```
    Wrap(Bracketed<Pointer<TypeValue>>),

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
        #[range_or = "parameters"]
        generics: Option<Pointer<GenericDeclaration>>,

        /// The parameters this function accepts.
        parameters: BracketedList<ParameterTypeName>,

        /// The `->` character.
        arrow: Token,

        /// The return type of this function
        return_type: Pointer<TypeValue>,
    },

    /// A reference to a different type.
    ///
    /// ```lua
    /// type Foo = Bar
    /// type FooBar = Qux<string>
    /// ```
    Basic {
        /// The name of the type that has the generics.
        base: Token,

        /// Optional generics.
        #[range_or = "base"]
        generics: Option<Pointer<BracketedList<Pointer<TypeValue>>>>,
    },

    /// A generic pack.
    ///
    /// ```lua
    /// <T...>
    /// ```
    GenericPack {
        /// The name.
        name: Token,

        /// The `...` characters.
        ellipsis: Token,
    },

    /// An intersection between two types.
    ///
    /// ```lua
    /// type Foo = Bar & Qux
    /// ```
    Intersection {
        /// The type at the start.
        left: Pointer<TypeValue>,

        /// The `&` character.
        ampersand: Token,

        /// The type at the end.
        right: Pointer<TypeValue>,
    },

    /// An union between two types.
    ///
    /// ```lua
    /// type Foo = Bar & Qux
    /// ```
    Union {
        /// The type at the start.
        left: Pointer<TypeValue>,

        /// The `|` character.
        pipe: Token,

        /// The type at the end.
        right: Pointer<TypeValue>,
    },

    /// An access to an exported type from a module.
    Module {
        /// the name of the module.
        module: Token,

        /// The `.` between the module name and the type.
        dot: Token,

        /// The actual name of the type being accessed.
        name: Token,

        /// Optional generics.
        #[range_or = "name"]
        generics: Option<Pointer<BracketedList<Pointer<TypeValue>>>>,
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
        base: Pointer<TypeValue>,

        /// The `?` character.
        question_mark: Token,
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
        typeof_token: Token,

        /// The expression passed to `typeof`.
        inner: Bracketed<Pointer<Expression>>,
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
    Tuple(BracketedList<Pointer<TypeValue>>),

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
        ellipsis: Token,

        /// The actual type.
        type_info: Pointer<TypeValue>,
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
        ellipsis: Token,

        /// The name
        name: Token,
    },
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Range, Print)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum ParameterTypeName {
    Normal(Name),
    Type(TypeValue),
}

/// A struct for a type definition. Holds needed data to be able to write it back as valid
/// luau.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Range, Print)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct TypeDefinition {
    /// The `export` keyword.
    #[range_or = "type_keyword"]
    pub export_keyword: Option<Token>,

    /// The `type` keyword.
    pub type_keyword: Token,

    /// The generics for this type.
    pub generics: Option<Pointer<GenericDeclaration>>,

    /// The name of the type.
    pub type_name: Token,

    /// The `=` sign between the name and the actual value of the type.
    /// This will be `None` if this isn't it's own statement but rather
    /// in another place like parameter's type or a variable's type.
    pub equal_sign: Token,

    /// The actual [`value`](TypeValue) of the type.
    pub type_value: Pointer<TypeValue>,
}

/// Generics parameters used when referencing another type.
pub type GenericParameters = BracketedList<GenericParameterInfo>;

/// A generic declaration parameter used in [`generics declarations`](GenericDeclaration).
/// Can either be a name or a variadic pack.
#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord, Range, Print)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum GenericParameterInfo {
    /// This [`GenericParameterInfo`] had a syntax error.
    #[default]
    ERROR,

    /// A simple name, such as `T`.
    Name(Token),

    /// A variadic type pack: `T...`.
    Pack {
        /// The name of the type.
        name: Token,
        /// The `...` characters.
        ellipsis: Token,
    },
}

/// A generic declaration parameter used in [`generic declarations`](GenericDeclaration).
/// Consists of a [`parameter info`](GenericParameterInfo) and an optional default type.
#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord, Range, Print)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct GenericDeclarationParameter {
    /// The parameter passed as a generic type, can be a simple name or a generic pack.
    pub parameter: GenericParameterInfo,

    /// The equal symbol.
    pub equal: Option<Token>,

    /// The default type.
    #[range_or = "parameter"]
    pub default: Option<GenericParameterInfoDefault>,
}

/// Struct holding **default** values for generic arguments.
#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord, Range, Print)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum GenericParameterInfoDefault {
    #[default]
    ERROR,

    /// A simple name.
    ///
    /// ```lua
    /// type Foo<T = string> = "Foo"
    /// ```
    Name(Token),

    /// A generic pack.
    ///
    /// ```lua
    /// type Foo<T... = string...> = "Foo"
    /// type Bar<T... = ...string> = "Bar"
    /// type Qux<T... = (string, number)> = "Qux"
    /// ```
    Pack(TypeValue),
}

/// The generics used in a [`type definition`](TypeDefinition).
pub type GenericDeclaration = BracketedList<GenericDeclarationParameter>;

/// Possible errors converting from an expression to a type definition.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Range, Print)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
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
        operator: Token,

        /// The actual expression this operator is affecting.
        expression: Pointer<Expression>,
    },

    /// Binary expressions require metatables. If you wish to just use the value of the
    /// expression, pass the inner one and not the binary.
    BinaryExpression {
        /// The left expression.
        left: Pointer<Expression>,

        /// The operator between the expressions.
        operator: Token,

        /// The right expression.
        right: Pointer<Expression>,
    },
}
