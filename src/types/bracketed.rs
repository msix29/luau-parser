use luau_lexer::token::Token;

/// An item that must be surrounded by [`brackets`](Token).
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, Hash, PartialEq, PartialOrd, Eq, Ord)]
pub struct Bracketed<T> {
    /// The opening bracket.
    opening_bracket: Token,

    /// The actual item
    item: T,

    /// The closing bracket.
    closing_bracket: Token,
}
