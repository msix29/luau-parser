use luau_lexer::prelude::{Keyword, Lexer, ParseError, PartialKeyword, Symbol, Token, TokenType};

use crate::types::{Expression, Parse};

impl Parse for Expression {
    fn parse(token: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        
        todo!()
    }
}
