#[macro_use]
mod macros;

mod block;
mod bracketed;
mod cst;
mod expression;
mod function;
mod list;
mod literals;
mod name;
mod range;
mod value;

use luau_lexer::prelude::{Lexer, ParseError, Token};
use std::sync::Arc;

use crate::types::Parse;

impl<T: Parse> Parse for Arc<T> {
    fn parse(token: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        T::parse(token, lexer, errors).map(Self::new)
    }
}
