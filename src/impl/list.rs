use std::fmt::Debug;

use luau_lexer::prelude::{
    Keyword, Lexer, Literal, LuauString, ParseError, PartialKeyword, Symbol, Token, TokenType,
};
use crate::types::{BracketedList, List, Parse, ParseWithArgs};

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

impl<T: Parse> ParseWithArgs<Symbol> for BracketedList<T> {
    fn parse_with(
        token: Token,
        lexer: &mut Lexer,
        errors: &mut Vec<ParseError>,
        args: Symbol,
    ) -> Option<Self> {
        todo!()
    }
}
