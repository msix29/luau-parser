use crate::prelude::{Expression, TypeDefinition};

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
#[derive(Clone, Debug)]
pub enum TableKey {
    /// Simple key
    String(String),

    /// An expression, can only be used in definitions and not in types.
    Expression(Expression),

    /// A type definition, can only be used in other types and not definitions.
    Type(TypeDefinition),
}

/// A struct representing one table field. It'll always have a _[key](TableKey)_ and a
/// _[type](TypeDefinition)_ and on optional _[value](TableFieldValue)_, the value will
/// only be present if this is an actual table and not type definition for the table.
#[derive(Clone, Debug)]
pub struct TableField {
    pub key: Box<TableKey>,
    pub value: Option<Box<TableFieldValue>>,
    pub r#type: Box<TypeDefinition>,
}

/// A possible value for a _[table field](TableField)_.
#[derive(Clone, Debug)]
pub enum TableFieldValue {
    /// An _[expression](Expression)_, can be found in declarations of tables as variables
    /// only.
    Expression(Expression),

    /// A _[type](TypeDefinition)_, can be found in tyepe definitions only.
    Type(TypeDefinition),
}

/// Struct for table _[expression](Expression)_ enum.
#[derive(Clone, Debug)]
pub struct TableValue {
    /// The actual _[fields](TableField)_ of the table.
    pub fields: Box<Vec<TableField>>,
}
