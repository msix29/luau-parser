use luau_lexer::prelude::Token;
use luau_parser_derive::{Print, Range};
use std::ops::{Deref, DerefMut};

/// The [`EndOfFile`][luau_lexer::prelude::TokenType::EndOfFile] token. This
/// token is always the last token in a file. It'll only really be useful if
/// a file has no code but only comments - the comments will be stored in
/// [`token.leading_trivia`](Token::leading_trivia) and there's no other way
/// to detect such comments.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Range, Print)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct EndOfFile(Token);

impl EndOfFile {
    #[inline]
    /// Create a new [`EndOfFile`] from the passed [`Token`]. Since this function is
    /// only available to the crate (`pub(crate)`), it does not validate where or
    /// not the passed [`Token`] is a
    /// [`TokenType::EndOfFile`](luau_lexer::prelude::TokenType::EndOfFile).
    pub(crate) fn new(token: Token) -> Self {
        Self(token)
    }
}

impl Deref for EndOfFile {
    type Target = Token;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for EndOfFile {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
