//! Implementation for literal types.

use std::ops::Deref;

use crate::prelude::{SingleToken, StringLiteral};
#[cfg(feature = "raw-value")]
use crate::types::HasRawValue;

/// Implements the [`Deref`] trait for a literal type.
macro_rules! __impl_deref_literal {
    ($struct: ident) => {
        impl Deref for $struct {
            type Target = SingleToken;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    };
}
__impl_deref_literal!(StringLiteral);

/// Implements the [`From`] trait for a struct representing a literal type, it
/// only calls that trait function from the inner [`SingleToken`].
macro_rules! __impl_from_node_literal {
    ($struct: ident) => {
        impl<T> From<T> for $struct
        where
            SingleToken: From<T>,
        {
            fn from(value: T) -> Self {
                Self(SingleToken::from(value))
            }
        }
    };
}
__impl_from_node_literal!(StringLiteral);

/// Removes the first and last characters of the string.
fn remove_surrounding_pair(string: &str) -> String {
    string[1..(string.len() - 1)].to_string()
}

impl StringLiteral {
    /// Removes string delimeters from a string. String delimeters are one of:
    /// * `"`
    /// * `'`
    /// * `` ` ``
    /// * `[[` and the matching `]]` (with _n_ `=`).
    pub fn strip_delimiters(string: &str) -> String {
        if string.starts_with('"') | string.starts_with('\'') | string.starts_with('`') {
            remove_surrounding_pair(string)
        } else if string.starts_with('[') {
            // Remove the outermost `[]`, then trim `=` at both ends, then finally the
            // innermost `[]`.
            remove_surrounding_pair(remove_surrounding_pair(string).trim_matches('='))
        } else {
            string.to_string()
        }
    }
}
