use luau_lexer::{
    prelude::{Lexer, ParseError, Token},
    token::{Keyword, Symbol, TokenType},
};

use crate::{
    types::{DoBlock, Expression, Name, NumericalFor, Parse, Pointer, TryParse},
    utils::get_token_type_display_extended,
};

impl Parse for NumericalFor {
    fn parse(for_keyword: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        if for_keyword != TokenType::Keyword(Keyword::For) {
            return None;
        }

        let variable = Name::try_parse(lexer, errors).unwrap_or(Name::ERROR);

        next_token_recoverable!(
            lexer,
            equal_keyword,
            TokenType::Symbol(Symbol::Equal),
            TokenType::Symbol(Symbol::Equal),
            errors,
            "Expected ".to_string()
                + get_token_type_display_extended(&TokenType::Symbol(Symbol::Equal))
        );

        let start = Pointer::<Expression>::try_parse(lexer, errors).unwrap_or_default();
        next_token_recoverable!(
            lexer,
            start_comma,
            TokenType::Symbol(Symbol::Comma),
            TokenType::Symbol(Symbol::Comma),
            errors,
            "Expected ".to_string()
                + get_token_type_display_extended(&TokenType::Symbol(Symbol::Comma))
        );

        let end = Pointer::<Expression>::try_parse(lexer, errors).unwrap_or_default();
        maybe_next_token!(lexer, end_comma, TokenType::Symbol(Symbol::Comma));

        let step = end_comma
            .as_ref()
            .and_then(|_| Pointer::<Expression>::try_parse(lexer, errors));
        let do_block = DoBlock::try_parse(lexer, errors)?;

        Some(Self {
            for_keyword,
            variable,
            equal_keyword,
            start,
            start_comma,
            end,
            end_comma,
            step,
            do_block,
        })
    }
}
