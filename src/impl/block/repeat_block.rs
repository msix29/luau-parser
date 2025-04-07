//! All `impl` blocks for [`RepeatBlock`].

use luau_lexer::prelude::{Keyword, Lexer, ParseError, Token, TokenType};

use crate::{
    safe_unwrap,
    types::{Block, Expression, Parse, Pointer, RepeatBlock, TryParse, TryParseWithArgs},
    utils::get_token_type_display_extended,
};

impl Parse for RepeatBlock {
    fn parse(
        repeat_keyword: Token,
        lexer: &mut Lexer,
        errors: &mut Vec<ParseError>,
    ) -> Option<Self> {
        if repeat_keyword != TokenType::Keyword(Keyword::Repeat) {
            return None;
        }

        let body = Block::try_parse_with(lexer, errors, TokenType::Keyword(Keyword::End))
            .unwrap_or_default();

        next_token_recoverable!(
            lexer,
            until_keyword,
            TokenType::Keyword(Keyword::Until),
            TokenType::Keyword(Keyword::Until),
            errors,
            "Expected ".to_string()
                + get_token_type_display_extended(&TokenType::Keyword(Keyword::Until))
        );

        let condition = safe_unwrap!(
            lexer,
            errors,
            "Expected <expr>",
            Pointer::<Expression>::try_parse(lexer, errors)
        );

        Some(Self {
            repeat_keyword,
            body,
            until_keyword,
            condition,
        })
    }
}
