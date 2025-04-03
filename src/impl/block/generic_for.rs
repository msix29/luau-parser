use luau_lexer::prelude::{Keyword, Lexer, ParseError, Token, TokenType};

use crate::types::{DoBlock, GenericFor, List, Parse, TryParse};

impl Parse for GenericFor {
    fn parse(for_keyword: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        let state = lexer.save_state();

        if for_keyword != TokenType::Keyword(Keyword::For) {
            return None;
        }

        let names = List::try_parse(lexer, errors)?;

        maybe_next_token!(lexer, in_keyword, TokenType::Keyword(Keyword::In));
        let Some(in_keyword) = in_keyword else{
            lexer.set_state(state);

            return None;
        };

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
