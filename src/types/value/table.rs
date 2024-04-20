//! Holding all needed information for tables.

use std::sync::Arc;

use smol_str::SmolStr;

use crate::prelude::{Expression, List, SingleToken, TypeDefinition};

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
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum TableKey {
    /// Cases in which a key wasn't provided, it's guessed as a number in that case.
    UndefinedNumber(i32),

    /// Cases in which a key wasn't provided, it's guessed as `[number]`. It has no other
    /// possible values than `[number]`.
    UndefinedString(SmolStr),

    /// Simple key
    String(SingleToken),

    /// An expression, can only be used in definitions and not in types.
    Expression {
        /// The `[` character.
        open_square_brackets: SingleToken,

        /// The actual expression between the `[...]`.
        expression: Arc<Expression>,

        /// The `]` character.
        close_square_brackets: SingleToken,
    },

    /// A type definition, can only be used in other types and not definitions.
    Type {
        /// The `[` character.
        open_square_brackets: SingleToken,

        /// The actual type between the `[...]`.
        r#type: Arc<TypeDefinition>,

        /// The `]` character.
        close_square_brackets: SingleToken,
    },
}

/// A struct representing one table field. It'll always have a [`key`](TableKey) and a
/// value that's either a [`type`](TypeDefinition) or an [`expression`](Expression). See
/// [`table field values`](TableFieldValue).
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct TableField {
    /// The [`key`](TableKey) used to index field.
    pub key: Arc<TableKey>,

    /// The `=` or `:` tokens, it's `=` in variables and `:` in types.
    pub equal_or_colon: Option<SingleToken>,

    /// The value of theis field. An expression in variables and a type in type
    /// definitions.
    pub value: Arc<TableFieldValue>,
}

/// A possible value for a [`table field`](TableField).
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum TableFieldValue {
    /// An [`expression`](Expression), can be found in declarations of tables as variables
    /// only.
    Expression(Expression),

    /// A [`type`](TypeDefinition), can be found in type definitions only.
    Type(TypeDefinition),
}

/// Struct representing a luau table.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Table {
    /// The `{` character.
    pub opening_brackets: SingleToken,

    /// The actual [`fields`](TableField) of the table.
    pub fields: List<TableField>,

    /// The `}` character.
    pub closing_brackets: SingleToken,
}
