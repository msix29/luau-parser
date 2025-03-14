//! # Name
//!
//! Module holding type definition for any grammar item related to names. Ex. variable
//! names.
//!

use luau_lexer::prelude::Token;
use std::sync::Arc;

use super::TypeValue;

/// A struct that provides a high level abstraction of `name` and `typedName` from the
/// grammar for easier usability..
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct NormalizedName {
    /// The actual name.
    pub name: Token,

    /// `:` character.
    pub colon: Option<Token>,

    /// The type that was with this name, defined with the `: type` syntax.
    pub r#type: Option<Arc<TypeValue>>,
}
