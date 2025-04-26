//! All `impl` blocks for [`EndOfFile`].

use luau_lexer::prelude::{Lexer, ParseError, Token, TokenType};

use crate::types::{EndOfFile, Parse};

impl Parse for EndOfFile {
    #[inline]
    fn parse(token: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        (token == TokenType::EndOfFile).then(|| EndOfFile::new(token))
    }
}
