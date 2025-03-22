use luau_lexer::prelude::{Lexer, ParseError, Symbol, Token, TokenType};
use std::sync::Arc;

use crate::{
    handle_error_token,
    types::{
        ExpressionWrap, Parse, ParseWithArgs, TableAccess, TableAccessKey, TableAccessPrefix,
        TableKey,
    },
};

impl Parse for TableAccessPrefix {
    fn parse(token: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        match token.token_type {
            TokenType::Error(error) => handle_error_token!(errors, error),
            TokenType::Identifier(_) | TokenType::PartialKeyword(_) => {
                Some(Self::Name(token)) // FunctionCall::parse(token.clone(), lexer, errors)
                                        //     .map(Arc::new)
                                        //     .map(Self::FunctionCall)
                                        //     .or(Some(Self::Name(token)))
            }
            TokenType::Symbol(Symbol::OpeningParenthesis) => ExpressionWrap::parse_with(
                token,
                lexer,
                errors,
                ("Expected <expr>", Symbol::ClosingParenthesis),
            )
            .map(Arc::new)
            .map(Self::ExpressionWrap),
            _ => None,
        }
    }
}

impl Parse for TableAccessKey {
    fn parse(maybe_dot: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        match maybe_dot.token_type {
            TokenType::Error(error) => handle_error_token!(errors, error),
            TokenType::Symbol(Symbol::Dot) => {
                next_token_recoverable!(
                    lexer,
                    name,
                    TokenType::Identifier(_) | TokenType::PartialKeyword(_),
                    TokenType::Identifier("*error*".to_string()),
                    errors,
                    "Expected <ident>"
                );

                Some(Self::Name {
                    dot: Box::new(maybe_dot),
                    name: Box::new(name),
                })
            }
            _ => TableKey::parse(maybe_dot, lexer, errors)
                .map(Box::new)
                .map(Self::Expression),
        }
    }
}

impl Parse for Vec<TableAccessKey> {
    fn parse(mut token: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        let mut keys = Vec::new();
        let mut state = lexer.save_state();

        while let Some(key) = TableAccessKey::parse(token, lexer, errors) {
            keys.push(key);
            state = lexer.save_state();
            token = lexer.next_token();
        }

        lexer.set_state(state);

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
