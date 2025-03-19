use luau_lexer::prelude::{
    Keyword, Lexer, Literal, ParseError, PartialKeyword, Symbol, Token, TokenType,
};

use crate::types::{Parse, TableAccess, TableAccessKey, TableAccessPrefix};

impl Parse for TableAccessPrefix {
    fn parse(token: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        todo!()
    }
}

impl Parse for TableAccessKey {
    fn parse(token: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        todo!()
    }
}
impl Parse for Vec<TableAccessKey> {
    fn parse(mut token: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        let mut keys = Vec::new();

        while let Some(key) = TableAccessKey::parse(token, lexer, errors) {
            keys.push(key);
            token = lexer.next_token();
        }

        (!keys.is_empty()).then_some(keys)
    }
}

impl Parse for TableAccess {
    fn parse(token: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        Some(Self {
            prefix: Parse::parse(token, lexer, errors)?,
            accessed_keys: Parse::parse(lexer.next_token(), lexer, errors)?,
        })
    }
}
