//! All `impl` blocks for [`Bracketed`].

use luau_lexer::prelude::{Lexer, ParseError, Symbol, Token, TokenType};
use std::ops::{Deref, DerefMut};

use crate::{
    types::{Bracketed, Parse, ParseWithArgs, Print},
    utils::get_token_type_display_extended,
};

impl<T: Default> Bracketed<T> {
    /// The actual parsing logic.
    fn parse<F>(
        parse: F,
        opening_bracket: Token,
        lexer: &mut Lexer,
        errors: &mut Vec<ParseError>,
        (error_message, stop_at): (&str, Symbol),
    ) -> Option<Self>
    where
        F: FnOnce(Token, &mut Lexer, &mut Vec<ParseError>) -> Option<T>,
    {
        let token = lexer.next_token();
        if token == TokenType::Symbol(stop_at) {
            return Some(Self {
                opening_bracket,
                item: T::default(),
                closing_bracket: token,
            });
        }

        let Some(item) = parse(token, lexer, errors) else {
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
            format!(
                "Expected {}",
                get_token_type_display_extended(&TokenType::Symbol(stop_at))
            )
        );

        Some(Self {
            opening_bracket,
            item,
            closing_bracket,
        })
    }
}

impl<T: Parse + Default> ParseWithArgs<(&str, Symbol)> for Bracketed<T> {
    #[inline]
    fn parse_with(
        opening_bracket: Token,
        lexer: &mut Lexer,
        errors: &mut Vec<ParseError>,
        (error_message, stop_at): (&str, Symbol),
    ) -> Option<Self> {
        Self::parse(
            |token: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>| {
                T::parse(token, lexer, errors)
            },
            opening_bracket,
            lexer,
            errors,
            (error_message, stop_at),
        )
    }
}
impl<A, T> ParseWithArgs<(&str, Symbol, A)> for Bracketed<T>
where
    T: ParseWithArgs<A> + Default,
{
    #[inline]
    fn parse_with(
        opening_bracket: Token,
        lexer: &mut Lexer,
        errors: &mut Vec<ParseError>,
        (error_message, stop_at, args): (&str, Symbol, A),
    ) -> Option<Self> {
        Self::parse(
            |token: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>| {
                T::parse_with(token, lexer, errors, args)
            },
            opening_bracket,
            lexer,
            errors,
            (error_message, stop_at),
        )
    }
}

impl<T> Deref for Bracketed<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.item
    }
}

impl<T> DerefMut for Bracketed<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.item
    }
}

impl<T: Print> Print for Bracketed<T> {
    fn print_final_trivia(&self) -> String {
        self.closing_bracket.print_final_trivia()
    }

    fn print_without_final_trivia(&self) -> String {
        self.opening_bracket.print_without_final_trivia()
            + &self.item.print_without_final_trivia()
            + &self.closing_bracket.print_without_final_trivia()
    }
}
