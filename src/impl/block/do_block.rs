use luau_lexer::prelude::{Lexer, ParseError, Token};

use crate::types::{DoBlock, Parse};

impl Parse for DoBlock {
    fn parse(token: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        todo!()
    }
}
