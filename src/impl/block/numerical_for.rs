use luau_lexer::prelude::{Lexer, ParseError, Token};

use crate::types::{NumericalFor, Parse};

impl Parse for NumericalFor {
    fn parse(token: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        todo!()
    }
}
