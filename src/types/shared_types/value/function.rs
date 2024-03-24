use std::sync::Arc;

use crate::prelude::TypeDefinition;

/// A single parameter that a function accepts.
#[derive(Clone, Debug)]
pub struct FunctionParameter {
    /// The name of the parameter.
    pub name: String,

    /// The _[type](TypeDefinition)_ of the parameter.
    pub r#type: Arc<TypeDefinition>,

    /// Whether or not this parameter is variadic.
    pub is_variadic: bool,
}

/// A single return that a function has.
#[derive(Clone, Debug)]
pub struct FunctionReturn {
    /// The _[type](TypeDefinition)_ of the return.
    pub r#type: Arc<TypeDefinition>,

    /// Whether or not this return is variadic.
    pub is_variadic: bool,
}

/// The actual value representing a function for the _[value](crate::prelude::Value)_ enum.
#[derive(Clone, Debug)]
pub struct FunctionValue {
    /// All _[parameters](FunctionParameter)_ of the function.
    pub parameters: Arc<Vec<FunctionParameter>>,

    /// All _[returns](FunctionReturn)_ of the function
    pub returns: Arc<Vec<FunctionReturn>>,
}
