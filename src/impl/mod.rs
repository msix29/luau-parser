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

use crate::types::{Parse, ParseWithArgs, Pointer, TryParse, TryParseWithArgs};

impl<T: Parse> Parse for Pointer<T> {
    #[inline]
    fn parse(token: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        T::parse(token, lexer, errors).map(Self::new)
    }
}
impl<T: TryParse + Parse> TryParse for Pointer<T> {
    #[inline]
    fn try_parse(lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        T::try_parse(lexer, errors).map(Self::new)
    }
}

impl<T: Parse> Parse for Box<T> {
    #[inline]
    fn parse(token: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        T::parse(token, lexer, errors).map(Self::new)
    }
}
impl<T: TryParse + Parse> TryParse for Box<T> {
    #[inline]
    fn try_parse(lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        T::try_parse(lexer, errors).map(Self::new)
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
impl<T: TryParse + Parse> TryParse for Vec<T> {}

impl<T: ParseWithArgs<A>, A: Clone> ParseWithArgs<A> for Vec<T> {
    #[inline]
    fn parse_with(
        mut token: Token,
        lexer: &mut Lexer,
        errors: &mut Vec<ParseError>,
        args: A,
    ) -> Option<Self> {
        let mut values = Vec::new();
        let mut state = lexer.save_state();

        while let Some(value) = T::parse_with(token, lexer, errors, args.clone()) {
            values.push(value);
            state = lexer.save_state();
            token = lexer.next_token();
        }

        lexer.set_state(state);

        (!values.is_empty()).then_some(values)
    }
}
impl<T: ParseWithArgs<A>, A: Clone> TryParseWithArgs<A> for T {}
