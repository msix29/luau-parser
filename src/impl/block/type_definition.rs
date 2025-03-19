use luau_lexer::prelude::{Lexer, ParseError, Token};

use crate::types::{
    GenericDeclaration, GenericDeclarationParameter, GenericParameterInfo,
    GenericParameterInfoDefault, GenericParameters, Parse, TypeDefinition, TypeValue,
};

impl Parse for TypeValue {
    fn parse(token: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        todo!()
    }
}

impl Parse for TypeDefinition {
    fn parse(token: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        todo!()
    }
}

impl Parse for GenericParameters {
    fn parse(token: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        todo!()
    }
}

impl Parse for GenericParameterInfo {
    fn parse(token: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        todo!()
    }
}

impl Parse for GenericDeclarationParameter {
    fn parse(token: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        todo!()
    }
}

impl Parse for GenericParameterInfoDefault {
    fn parse(token: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        todo!()
    }
}

impl Parse for GenericDeclaration {
    fn parse(token: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        todo!()
    }
}
