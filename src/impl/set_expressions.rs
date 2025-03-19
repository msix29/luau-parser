use luau_lexer::prelude::{Lexer, ParseError, Token};

use crate::types::{CompoundSetExpression, Parse, SetExpression};

impl Parse for SetExpression {
    fn parse(token: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        todo!()
    }
}

impl Parse for CompoundSetExpression {
    fn parse(token: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        todo!()
    }
}
