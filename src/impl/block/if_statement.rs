use luau_lexer::prelude::{Lexer, ParseError, Token};

use crate::types::{ElseIfStatement, ElseStatement, IfStatement, Parse};

impl Parse for IfStatement {
    fn parse(token: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        todo!()
    }
}

impl Parse for ElseIfStatement {
    fn parse(token: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        todo!()
    }
}

impl Parse for ElseStatement {
    fn parse(token: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        todo!()
    }
}
