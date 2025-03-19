use luau_lexer::prelude::{Lexer, ParseError, Token};

use crate::types::{LocalAssignment, Parse};

impl Parse for LocalAssignment {
    fn parse(token: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        todo!()
    }
}
