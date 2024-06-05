//! # Name
//!
//! Module holding type definition for any grammar item related to names. Ex. variable
//! names.
//!

use std::sync::Arc;

#[cfg(feature = "references")]
use super::references::References;
use super::{Token, TypeValue};
use crate::generate_derives;

generate_derives! {
    Default,
    /// A struct that provides a high level abstraction of `name` and `typedName` from the
    /// grammar for easier usability..
    pub struct NormalizedName {
        /// The actual name.
        pub name: Token,

        /// The type that was with this name, defined with the `: type` syntax.
        pub colon: Option<Token>,

        /// The type that was with this name, defined with the `: type` syntax.
        pub r#type: Option<Arc<TypeValue>>,

        /// The type that was with this name, defined with the `: type` syntax.
        #[cfg(feature = "references")]
        pub references: References,
    }
}
