use luau_lexer::prelude::{Lexer, ParseError, Token};

use crate::types::{GlobalFunction, GlobalFunctionName, LocalFunction, Parse};

impl Parse for LocalFunction {
    fn parse(token: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        todo!()
    }
}

impl Parse for GlobalFunctionName {
    fn parse(token: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        todo!()
    }
}

impl Parse for GlobalFunction {
    fn parse(token: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        todo!()
    }
}
