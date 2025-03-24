use luau_lexer::token::Token;

// A comment.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Comment(pub Token);
