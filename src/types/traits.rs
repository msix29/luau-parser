//! Module holding all trait definitions in this parser.

use luau_lexer::prelude::{Lexer, ParseError, Token};

use crate::types::Range;

/// A trait for a token that can be represented in a more abstract form for the user to see,
/// without maintaing original styling. This is mainly for LSPs so it's LSP-ready and can
/// be used for things like hover.
#[cfg(feature = "raw-values")]
pub trait HasRawValue {
    /// Get the lossy _raw value_ of this token. For lossless, see [`print`](Print).
    fn get_raw_value(&self) -> String;
}

/// A trait to print the token as-is, while preserving all user spaces and styling.
pub trait Print {
    /// Prints the whole token including all surrounding spaces.
    fn print(&self) -> String;
}

/// A trait that indicates that this struct can be parsed from a [`lexer`](Lexer)
pub trait Parse<O = Self> {
    fn parse(token: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<O>;
}
pub(crate) trait TryParse<O = Self>
where
    O: Parse<O>,
{
    fn try_parse(lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<O> {
        let state = lexer.save_state();

        match O::parse(lexer.next_token(), lexer, errors) {
            value @ Some(_) => value,
            None => {
                lexer.set_state(state);

                None
            }
        }
    }
}

/// A trait that means this node can be built from a [`tree-sitter Node`](Node).
pub trait ParseWithArgs<T, O = Self> {
    fn parse_with(
        token: Token,
        lexer: &mut Lexer,
        errors: &mut Vec<ParseError>,
        args: T,
    ) -> Option<O>;
}
pub(crate) trait TryParseWithArgs<T, O = Self, O2 = O>
where
    O2: ParseWithArgs<T, O>,
{
    #[inline]
    fn try_parse_with(lexer: &mut Lexer, errors: &mut Vec<ParseError>, args: T) -> Option<O> {
        let state = lexer.save_state();

        match O2::parse_with(lexer.next_token(), lexer, errors, args) {
            value @ Some(_) => value,
            None => {
                lexer.set_state(state);

                None
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum GetRangeError {
    ErrorVariant,
}

/// A trait for getting the range for this specific item.
pub trait GetRange {
    /// Get the range of the node.
    fn get_range(&self) -> Result<Range, GetRangeError>;
}
