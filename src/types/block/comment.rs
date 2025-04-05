use luau_lexer::prelude::Token;
use luau_parser_derive::{Print, Range};

// A comment.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Range, Print)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Comment(pub Token);
