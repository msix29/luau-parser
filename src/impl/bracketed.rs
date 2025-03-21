use luau_lexer::prelude::{Lexer, ParseError, Symbol, Token, TokenType};

use crate::types::{Bracketed, Parse, ParseWithArgs};

impl<T: Parse> ParseWithArgs<(&str, Symbol)> for Bracketed<T> {
    fn parse_with(
        opening_bracket: Token,
        lexer: &mut Lexer,
        errors: &mut Vec<ParseError>,
        (error_message, stop_at): (&str, Symbol),
    ) -> Option<Self> {
        let Some(item) = T::parse(lexer.next_token(), lexer, errors) else {
            let state = lexer.save_state();
            errors.push(ParseError::new(
                state.lexer_position(),
                error_message.to_string(),
                Some(state.lexer_position()),
            ));

            return None;
        };

        next_token_recoverable_with_condition!(
            lexer,
            closing_bracket,
            closing_bracket.token_type == TokenType::Symbol(stop_at),
            TokenType::Symbol(stop_at),
            errors,
            "Expected <opening-parenthesis>"
        );

        Some(Self {
            opening_bracket,
            item,
            closing_bracket,
        })
    }
}
