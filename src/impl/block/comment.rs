use luau_lexer::prelude::{Lexer, ParseError, Token, TokenType};

use crate::types::{Comment, Parse};

impl Parse for Comment {
    #[inline]
    fn parse(token: Token, _: &mut Lexer, _: &mut Vec<ParseError>) -> Option<Self> {
        match token.token_type {
            TokenType::Comment(_) => Some(Self(token)),
            _ => None,
        }
    }
}
