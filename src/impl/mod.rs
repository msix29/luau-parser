#[macro_use]
mod macros;

mod block;
mod bracketed;
mod cst;
mod expression;
mod list;
mod literals;
mod name;
mod range;
mod value;

use luau_lexer::prelude::{Lexer, ParseError, Token};
use std::sync::Arc;

use crate::types::Parse;

impl<T: Parse> Parse for Arc<T> {
    #[inline]
    fn parse(token: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        T::parse(token, lexer, errors).map(Self::new)
    }
}

impl<T: Parse> Parse for Vec<T> {
    #[inline]
    fn parse(mut token: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        let mut values = Vec::new();
        let mut state = lexer.save_state();

        while let Some(value) = T::parse(token, lexer, errors) {
            values.push(value);
            state = lexer.save_state();
            token = lexer.next_token();
        }

        lexer.set_state(state);

        (!values.is_empty()).then_some(values)
    }
}
