use luau_lexer::prelude::{Keyword, Lexer, ParseError, Token, TokenType};

use crate::{
    types::{Block, DoBlock, Parse, TryParse, TryParseWithArgs},
    utils::get_token_type_display_extended,
};

impl Parse for DoBlock {
    fn parse(do_keyword: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        if do_keyword != TokenType::Keyword(Keyword::Do) {
            return None;
        }

        let body = Block::try_parse_with(lexer, errors, TokenType::Keyword(Keyword::End))
            .unwrap_or_default();

        next_token_recoverable!(
            lexer,
            end_keyword,
            TokenType::Keyword(Keyword::End),
            TokenType::Keyword(Keyword::End),
            errors,
            "Expected ".to_string()
                + get_token_type_display_extended(&TokenType::Keyword(Keyword::End))
        );

        Some(Self {
            do_keyword,
            body,
            end_keyword,
        })
    }
}
impl TryParse for DoBlock {}
