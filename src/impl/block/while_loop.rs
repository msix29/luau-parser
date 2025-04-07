//! All `impl` blocks for [`WhileLoop`].

use luau_lexer::prelude::{Keyword, Lexer, ParseError, Token, TokenType};

use crate::{
    safe_unwrap,
    types::{DoBlock, Expression, Parse, Pointer, TryParse, WhileLoop},
};

impl Parse for WhileLoop {
    fn parse(
        while_keyword: Token,
        lexer: &mut Lexer,
        errors: &mut Vec<ParseError>,
    ) -> Option<Self> {
        if while_keyword != TokenType::Keyword(Keyword::While) {
            return None;
        }

        let condition = safe_unwrap!(
            lexer,
            errors,
            "Expected <expr>",
            Pointer::<Expression>::try_parse(lexer, errors)
        );
        let do_block = DoBlock::try_parse(lexer, errors)?;

        Some(Self {
            while_keyword,
            condition,
            do_block,
        })
    }
}
