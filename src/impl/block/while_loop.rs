use luau_lexer::prelude::{Lexer, ParseError, Token};

use crate::types::{Parse, WhileLoop};

impl Parse for WhileLoop {
    fn parse(token: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        todo!()
    }
}
