use crate::prelude::TypeDefinition;

#[derive(Clone, Debug)]
pub struct FunctionParameter {
    pub name: String,
    pub r#type: TypeDefinition,
    pub is_variadic: bool,
}

#[derive(Clone, Debug)]
pub struct FunctionReturn {
    pub r#type: TypeDefinition,
    pub is_variadic: bool,
}

#[derive(Clone, Debug)]
pub struct FunctionValue {
    pub parameters: Vec<FunctionParameter>,
    pub returns: Vec<FunctionReturn>,
}
