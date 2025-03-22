use luau_lexer::prelude::{Lexer, Literal, ParseError, Symbol, Token, TokenType};
use std::sync::Arc;

use crate::{
    types::{
        BracketedList, FunctionArguments, FunctionCall, FunctionCallInvoked, Parse, ParseWithArgs,
        PrefixExp, Table, TableAccessPrefix,
    },
    utils::try_parse,
};

impl Parse for FunctionCallInvoked {
    fn parse(token: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        let prefix_exp = Arc::new(PrefixExp::parse_with(token, lexer, errors, true)?);
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
            invoked: try_parse(lexer.save_state(), token, lexer, errors)?,
            arguments: try_parse(lexer.save_state(), lexer.next_token(), lexer, errors)?,
        })
    }
}

impl Parse<PrefixExp> for FunctionCall {
    fn parse(token: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<PrefixExp> {
        Self::parse(token, lexer, errors).map(PrefixExp::FunctionCall)
    }
}
impl Parse<TableAccessPrefix> for FunctionCall {
    fn parse(
        token: Token,
        lexer: &mut Lexer,
        errors: &mut Vec<ParseError>,
    ) -> Option<TableAccessPrefix> {
        Self::parse(token, lexer, errors)
            .map(Arc::new)
            .map(TableAccessPrefix::FunctionCall)
    }
}

impl Parse for FunctionArguments {
    fn parse(token: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        if matches!(token.token_type, TokenType::Literal(Literal::String(_))) {
            return Some(Self::String(token));
        }
        if token.token_type == TokenType::Symbol(Symbol::OpeningParenthesis) {
            return BracketedList::parse_with(
                token,
                lexer,
                errors,
                ("Expected <expr>", Symbol::ClosingParenthesis),
            )
            .map(Self::List);
        }

        Table::parse(token.clone(), lexer, errors)
    }
}
