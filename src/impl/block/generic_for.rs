use luau_lexer::prelude::{Keyword, Lexer, ParseError, Token, TokenType};

use crate::{
    types::{DoBlock, GenericFor, List, Parse, TryParse},
    utils::get_token_type_display_extended,
};

impl Parse for GenericFor {
    fn parse(for_keyword: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        if for_keyword != TokenType::Keyword(Keyword::For) {
            return None;
        }

        let names = List::try_parse(lexer, errors)?;

        next_token_recoverable!(
            lexer,
            in_keyword,
            TokenType::Keyword(Keyword::In),
            TokenType::Keyword(Keyword::In),
            errors,
            "Expected ".to_string()
                + get_token_type_display_extended(&TokenType::Keyword(Keyword::In))
        );

        let expressions = List::try_parse(lexer, errors)?;
        let do_block = DoBlock::try_parse(lexer, errors)?;

        Some(Self {
            for_keyword,
            names,
            in_keyword,
            expressions,
            do_block,
        })
    }
}
