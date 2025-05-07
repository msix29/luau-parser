//! Module holding all trait definitions in this crate.

use luau_lexer::prelude::{Lexer, ParseError, Token};
use lsp_types::Range;

/// A trait to print the token as-is, while preserving all user spaces, comments
/// and styling.
pub trait Print {
    /// Prints only the very final trivia. Used for the default implementation of
    /// [`Print::print`], which just joins [`Print::print_without_final_trivia`]
    /// and this function.
    fn print_final_trivia(&self) -> String;

    /// Prints the whole token including all surrounding trivia, excluding the
    /// very last trailing trivia.
    fn print_without_final_trivia(&self) -> String;

    /// Prints the whole token including all surrounding trivia.
    #[inline]
    fn print(&self) -> String {
        self.print_without_final_trivia() + &self.print_final_trivia()
    }
}

/// A trait that to parse this struct from a [`lexer`](Lexer) and starting with
/// a specific [`token`](Token).
pub trait Parse<O = Self> {
    /// Try parsing the current item, starting from the passed token.
    fn parse(token: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<O>;
}

/// A wrapper trait for [`Parse`] where it would reset the lexer's state upon
/// failure.
pub(crate) trait TryParse<O = Self>
where
    O: Parse<O>,
{
    /// Try parsing and reset the lexer's state upon failure.
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

/// A trait that to parse this struct from a [`lexer`](Lexer) and starting with
/// a specific [`token`](Token) and with specific arguments.
pub trait ParseWithArgs<T, O = Self> {
    /// Try parsing the current item, starting from the passed token with the
    /// passed arguments.
    fn parse_with(
        token: Token,
        lexer: &mut Lexer,
        errors: &mut Vec<ParseError>,
        args: T,
    ) -> Option<O>;
}

/// A wrapper trait for [`ParseWithArgs`] where it would reset the lexer's state
/// upon failure.
pub(crate) trait TryParseWithArgs<T, O = Self, O2 = O>
where
    O2: ParseWithArgs<T, O>,
{
    /// Try parsing and reset the lexer's state upon failure.
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

/// Errors that may occur during [`get_range`](GetRangeError). They should
/// never happen if [`Cst.status`](crate::types::Cst::status) is
/// [`AstStatus::Complete`](crate::types::AstStatus::Complete).
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum GetRangeError {
    /// This is an `ERROR` variant and thus spans no range in the source code.
    ErrorVariant,

    /// This is an empty list and thus spans no range in the source code.
    EmptyList,

    /// This is an empty block and thus spans no range in the source code.
    EmptyBlock,

    /// This is either
    /// * [`TableKey::UndefinedString`](crate::types::TableKey::UndefinedString), or
    /// * [`TableKey::UndefinedNumber`](crate::types::TableKey::UndefinedNumber)
    ///
    /// which don't actually exist in the source code and are added by the parser.
    UndefinedKey,
}

/// A trait for getting the range for this specific item.
pub trait GetRange {
    /// Get the range of the node. This will only fail if
    /// [`Cst.status`](crate::types::Cst::status) is
    /// [`AstStatus::HasErrors`](crate::types::AstStatus::HasErrors).
    fn get_range(&self) -> Result<Range, GetRangeError>;
}
