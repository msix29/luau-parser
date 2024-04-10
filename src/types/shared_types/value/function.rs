//! Holding function parameters.

use std::sync::Arc;

use crate::prelude::{SingleToken, TypeDefinition};

/// A single parameter that a function accepts.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct FunctionParameter {
    /// The name of the parameter.
    pub name: SingleToken,

    /// The [`type`](TypeDefinition) of the parameter.
    pub r#type: Option<Arc<TypeDefinition>>,
}
