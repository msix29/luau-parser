use luau_lexer::prelude::{Lexer, ParseError, Token};

use crate::types::{Name, Parse};

impl Parse for Name {
    fn parse(token: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        todo!()
    }
}
