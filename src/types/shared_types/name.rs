use crate::prelude::TypeDefinition;

#[derive(Clone, Debug, Default)]
pub struct NormalizedName {
    pub name: String,
    pub r#type: Option<TypeDefinition>,
    pub is_type_optional: bool,
}
