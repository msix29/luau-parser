use luau_lexer::prelude::{Lexer, ParseError, Token};

use crate::types::{Parse, RepeatBlock};

impl Parse for RepeatBlock {
    fn parse(token: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        todo!()
    }
}
