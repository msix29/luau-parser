//! Implementation for literal types.

use std::ops::Deref;
use tree_sitter::Node;

#[cfg(feature = "raw-values")]
use crate::types::HasRawValue;
use crate::{
    types::{FromNode, Number, ParseNumberError, ParsedNumber, StringLiteral, Token},
    utils::remove_surrounding_pair,
};

/// Implements the [`Deref`] trait for a literal type.
macro_rules! __impl_deref_literal {
    ($struct: ident) => {
        impl Deref for $struct {
            type Target = Token;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    };
}
__impl_deref_literal!(StringLiteral);
__impl_deref_literal!(Number);

/// Implements the [`From`] trait for a struct representing a literal type, it
/// only calls that trait function from the inner [`Token`].
macro_rules! __impl_from_node_literal {
    ($struct: ident) => {
        impl FromNode for $struct {
            fn from_node(node: Node, code_bytes: &[u8]) -> Option<Self> {
                Some(Self(Token::from_node(node, code_bytes)?))
            }
        }
    };
}
__impl_from_node_literal!(StringLiteral);
__impl_from_node_literal!(Number);

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

impl Number {
    /// Parses the number into [`ParsedNumber`]. That enum is used instead of a fixed
    /// number since hexadecimal and binary numbers have different ranges than the rest
    /// of the numers in Luau.
    pub fn parse(&self) -> Result<ParsedNumber, ParseNumberError> {
        let number = self.0.word.replace('_', "");

        if let Some(number) = number.strip_prefix('0') {
            if let Some((number, radix)) = number
                .strip_prefix('x')
                .map(|number| (number, 16))
                .or(number.strip_prefix('b').map(|number| (number, 2)))
            {
                return i128::from_str_radix(number, radix)
                    .map(ParsedNumber::HexOrByte)
                    .map_err(ParseNumberError::HexOrByte);
            }
        }

        number
            .parse::<f64>()
            .map(ParsedNumber::Other)
            .map_err(ParseNumberError::Other)
    }
}
