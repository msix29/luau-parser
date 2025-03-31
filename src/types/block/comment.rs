use luau_lexer::token::Token;
use luau_parser_derive::Range;

// A comment.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Range)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Comment(pub Token);
