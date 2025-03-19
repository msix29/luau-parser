use luau_lexer::prelude::{Lexer, Literal, ParseError, Symbol, Token, TokenType};
use std::sync::Arc;

use crate::types::{
    BracketedList, FunctionArguments, FunctionCall, FunctionCallInvoked, Parse, ParseWithArgs,
    PrefixExp, Table,
};

impl Parse for FunctionCallInvoked {
    fn parse(token: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        let prefix_exp = Arc::new(PrefixExp::parse(token, lexer, errors)?);
        let state = lexer.save_state();

        let maybe_colon = lexer.next_token();

        if maybe_colon != TokenType::Symbol(Symbol::Colon) {
            lexer.set_state(state);

            return Some(Self::Function(prefix_exp));
        }

        Some(Self::TableMethod {
            table: prefix_exp,
            colon: Box::new(maybe_colon),
            method: Box::new(lexer.next_token()),
        })
    }
}

impl Parse for FunctionCall {
    fn parse(token: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        Some(Self {
            invoked: Parse::parse(token, lexer, errors)?,
            arguments: Parse::parse(lexer.next_token(), lexer, errors)?,
        })
    }
}

impl Parse for FunctionArguments {
    fn parse(token: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        if matches!(token.token_type, TokenType::Literal(Literal::String(_))) {
            return Some(Self::String(token));
        }
        if token.token_type == TokenType::Symbol(Symbol::OpeningParenthesis) {
            return BracketedList::parse_with(token, lexer, errors, Symbol::ClosingParenthesis)
                .map(Self::List);
        }

        Table::parse(token.clone(), lexer, errors).map(Self::Table)
    }
}
