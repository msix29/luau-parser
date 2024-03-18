use super::Value;

#[derive(Clone, Debug, Default)]
pub struct TypeValue {
    pub r#type: Value,
    pub and_types: &'static [Value],
    pub or_types: &'static [Value],
}

#[derive(Clone, Debug)]
pub struct TypeDefinition {
    pub type_name: String,
    pub is_exported: bool,
    pub type_value: TypeValue,
}
