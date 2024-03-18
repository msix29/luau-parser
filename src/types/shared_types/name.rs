//! # Name
//!
//! Module holding type definition for any grammar item related to names. Ex. variable
//! names.
//!

use crate::prelude::TypeDefinition;

/// A struct that provides a high level abstraction of `name` and `typedName` from the
/// grammar for easier usability..
#[derive(Clone, Debug, Default)]
pub struct NormalizedName {
    /// The actual name.
    pub name: String,

    /// The type that was with this name, defined with the `: type` syntax.
    pub r#type: Option<TypeDefinition>,

    /// Whether or not the type had the `?` at the end of it, meaning it can be `nil`.
    pub is_type_optional: bool,
}
