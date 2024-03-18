use crate::prelude::{TypeDefinition, Value};

#[derive(Clone, Debug)]
pub enum TableKey {
    String(String),
    Value(Value),
    Type(TypeDefinition),
}

#[derive(Clone, Debug)]
pub struct TableField {
    pub key: TableKey,
    pub value: Option<TableFieldValue>,
    pub r#type: TypeDefinition,
}

#[derive(Clone, Debug)]
pub enum TableFieldValue {
    Value(Value),
    Type(TypeDefinition),
}

#[derive(Clone, Debug)]
pub struct TableValue {
    pub fields: Vec<TableField>,
}
