//! Holding all needed information for tables.

use luau_lexer::prelude::Token;
use luau_parser_derive::{Print, Range};
use smol_str::SmolStr;

use crate::types::{Bracketed, BracketedList, Expression, Pointer, TypeValue};

/// A possible key entry in a table. The key is usually a string, but it can be a value
/// (from an expression) in tables or a type in type definitions.
///
/// ```lua
/// local t = {
///     -- The normal string key.
///     foo = "foo",
///
///     -- A Value key, it's still a string, but what's inside [] is
///     -- always counted as an expression, even if simple.
///     ["bar"] = "bar",
/// }
///
/// type T = {
///     -- The normal string key.
///     foo: "foo",
///
///     -- A Type key, it indicates that the key can be any string, not the word "string".
///     [string]: number,
/// }
/// ```
#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum TableKey {
    /// This [`TableKey`] had a syntax error.
    #[default]
    ERROR,

    /// Cases in which a key in a table expression wasn't provided,
    /// it's guessed as a number in that case.
    UndefinedNumber(u32),

    /// Cases in which a key in a type expression wasn't provided,
    /// it's guessed as `number`. It has no other possible values
    /// than `number`.
    UndefinedString(SmolStr),

    /// Simple key
    Simple(Token),

    /// An expression, can only be used in definitions and not in types.
    Expression(Bracketed<Pointer<Expression>>),

    /// A type definition, can only be used in other types and not definitions.
    Type(Bracketed<Pointer<TypeValue>>),
}

/// A struct representing one table field. It'll always have a [`key`](TableKey) and a
/// value that's either a [`type`](TypeDefinition) or an [`expression`](Expression). See
/// [`table field values`](TableFieldValue).
#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct TableField {
    /// The [`key`](TableKey) used to index field.
    pub key: Pointer<TableKey>,

    /// The `=` or `:` tokens, it's `=` in variables and `:` in types.
    pub equal_or_colon: Option<Token>,

    /// The value of this field. An expression in variables and a type in type
    /// definitions.
    pub value: Pointer<TableFieldValue>,
}

/// A possible value for a [`table field`](TableField).
#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord, Range, Print)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum TableFieldValue {
    /// This [`TableFieldValue`] had a syntax error.
    #[default]
    ERROR,

    /// An [`expression`](Expression), can be found in declarations of tables as variables
    /// only.
    Expression(Expression),

    /// A [`type`](TypeValue), can be found in type definitions only.
    Type(TypeValue),

    ///```lua
    /// {...}
    /// ```
    // only in expressions
    VariadicValues(Token),
}

/// Struct representing a luau table.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Range, Print)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Table(pub BracketedList<TableField>);
