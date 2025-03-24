use luau_lexer::prelude::{Keyword, Lexer, Literal, ParseError, Symbol, Token, TokenType};
use std::sync::Arc;

use crate::{
    force_parse_bracketed, parse_bracketed,
    types::{
        Block, BracketedList, Closure, FunctionArguments, FunctionCall, FunctionCallInvoked, Parse,
        ParseWithArgs, PrefixExp, Table, TableAccessPrefix, TypeValue,
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

impl Parse for Closure {
    fn parse(
        function_keyword: Token,
        lexer: &mut Lexer,
        errors: &mut Vec<ParseError>,
    ) -> Option<Self> {
        // It's only called when it matches, but the check must still be here.
        if function_keyword != TokenType::Keyword(Keyword::Function) {
            return None;
        }

        let generics = parse_bracketed!(
            lexer,
            errors,
            "Expected <generic declaration>",
            TokenType::Symbol(Symbol::OpeningAngleBrackets),
            Symbol::ClosingAngleBrackets,
        )
        .map(Box::new);

        let parameters = force_parse_bracketed!(
            lexer,
            errors,
            "Expected <opening parenthesis>",
            (
                TokenType::Symbol(Symbol::OpeningParenthesis),
                TokenType::Symbol(Symbol::OpeningParenthesis)
            ),
            Symbol::ClosingParenthesis,
        );

        maybe_next_token!(lexer, maybe_colon, TokenType::Symbol(Symbol::Colon));
        let return_type = if maybe_colon.is_some() {
            TypeValue::parse(lexer.next_token(), lexer, errors).map(Arc::new)
        } else {
            None
        };

        let body = Block::parse_with(
            lexer.next_token(),
            lexer,
            errors,
            Some(TokenType::Keyword(Keyword::End)),
        )
        .unwrap_or_default(); // `Block::parse` never fails.

        next_token_recoverable!(
            lexer,
            end_keyword,
            TokenType::Keyword(Keyword::End),
            TokenType::Keyword(Keyword::End),
            errors,
            "Expected <end>"
        );

        Some(Self {
            function_keyword,
            generics,
            parameters,
            colon: Box::new(maybe_colon),
            return_type,
            body,
            end_keyword: Box::new(end_keyword),
        })
    }
}
