use luau_lexer::token::Token;
use luau_parser_derive::Range;

/// An item that must be surrounded by [`brackets`](Token).
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, Hash, PartialEq, PartialOrd, Eq, Ord, Range)]
pub struct Bracketed<T> {
    /// The opening bracket.
    pub opening_bracket: Token,

    /// The actual item
    pub item: T,

    /// The closing bracket.
    pub closing_bracket: Token,
}
