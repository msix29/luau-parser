use luau_lexer::{
    prelude::{Lexer, ParseError, Token},
    token::{Keyword, TokenType},
};

use crate::types::{Block, DoBlock, Parse, ParseWithArgs};

impl Parse for DoBlock {
    fn parse(do_keyword: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        if do_keyword != TokenType::Keyword(Keyword::Do) {
            return None;
        }

        let body = Block::parse_with(
            lexer.next_token(),
            lexer,
            errors,
            TokenType::Keyword(Keyword::End),
        )
        .unwrap_or_default();

        next_token_recoverable!(
            lexer,
            end_keyword,
            TokenType::Keyword(Keyword::End),
            TokenType::Keyword(Keyword::End),
            errors,
            "Expected <end>"
        );

        Some(Self {
            do_keyword,
            body,
            end_keyword,
        })
    }
}
