use luau_lexer::prelude::{Lexer, ParseError, Token};

use crate::types::{
    Expression, FunctionArguments, Parse, Table, TableAccessKey, TableField, TableFieldValue,
    TableKey,
};

impl Parse for TableKey {
    fn parse(token: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        todo!()
    }
}

impl Parse for TableField {
    fn parse(token: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        todo!()
    }
}

impl Parse for TableFieldValue {
    fn parse(token: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        todo!()
    }
}

impl Parse for Table {
    fn parse(token: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        todo!()
    }
}

impl Parse<FunctionArguments> for Table {
    #[inline]
    fn parse(
        token: Token,
        lexer: &mut Lexer,
        errors: &mut Vec<ParseError>,
    ) -> Option<FunctionArguments> {
        Self::parse(token, lexer, errors).map(FunctionArguments::Table)
    }
}

impl Parse<TableAccessKey> for TableKey {
    #[inline]
    fn parse(
        token: Token,
        lexer: &mut Lexer,
        errors: &mut Vec<ParseError>,
    ) -> Option<TableAccessKey> {
        Self::parse(token, lexer, errors)
            .map(Box::new)
            .map(TableAccessKey::Expression)
    }
}
