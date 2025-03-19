use luau_lexer::prelude::{Lexer, ParseError, Token};

use crate::types::{GenericFor, Parse};

impl Parse for GenericFor {
    fn parse(token: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        todo!()
    }
}
