//! Holding all needed information for tables.

use std::sync::Arc;

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
    UndefinedString(String),

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

/// A struct representing one table field. It'll always have a _[key](TableKey)_ and a
/// _[type](TypeDefinition)_ and on optional _[value](TableFieldValue)_, the value will
/// only be present if this is an actual table and not type definition for the table.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct TableField {
    /// The key of this field.
    pub key: Arc<TableKey>,

    /// The `=` or `:` tokens, it's `=` in variables and `:` in types.
    pub equal_or_colon: Option<SingleToken>,

    /// The value of the variable, only exists if this table is a variable.
    pub value: Option<Arc<TableFieldValue>>,

    /// The type of this field, this is always present regardless of the field type.
    pub r#type: Option<Arc<TypeDefinition>>,
}

/// A possible value for a _[table field](TableField)_.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum TableFieldValue {
    /// An _[expression](Expression)_, can be found in declarations of tables as variables
    /// only.
    Expression(Expression),

    /// A _[type](TypeDefinition)_, can be found in type definitions only.
    Type(TypeDefinition),
}

/// Struct for table _[expression](Expression)_ enum.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct TableValue {
    /// The `{` character.
    pub opening_brackets: SingleToken,

    /// The actual _[fields](TableField)_ of the table.
    pub fields: List<TableField>,

    /// The `}` character.
    pub closing_brackets: SingleToken,
}
