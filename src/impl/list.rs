use luau_lexer::prelude::{Lexer, ParseError, Token};
use std::fmt::Debug;

pub use crate::types::{List, Parse};

impl<T> List<T> {
    #[inline]
    pub const fn new() -> Self {
        Self { items: Vec::new() }
    }
}

impl<T: Debug + Parse> Parse for List<T> {
    fn parse(token: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        todo!()
    }
}
