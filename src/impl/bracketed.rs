use luau_lexer::prelude::{Lexer, ParseError, Symbol, Token};

use crate::types::{Bracketed, Parse, ParseWithArgs};

impl<T: Parse> ParseWithArgs<Symbol> for Bracketed<T> {
    fn parse_with(
        token: Token,
        lexer: &mut Lexer,
        errors: &mut Vec<ParseError>,
        args: Symbol,
    ) -> Option<Self> {
        todo!()
    }
}
