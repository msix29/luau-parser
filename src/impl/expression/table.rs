use luau_lexer::prelude::{Lexer, ParseError, Symbol, Token, TokenType};

use crate::{
    handle_error_token,
    types::{
        ExpressionWrap, FunctionCall, Parse, ParseWithArgs, Pointer, TableAccess, TableAccessKey,
        TableAccessPrefix, TableKey, TryParse,
    },
};

impl Parse for TableAccessPrefix {
    fn parse(token: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        match token.token_type {
            TokenType::Error(error) => handle_error_token!(errors, error),
            TokenType::Identifier(_) | TokenType::PartialKeyword(_) => {
                FunctionCall::parse(token.clone(), lexer, errors).or(Some(Self::Name(token)))
            }
            TokenType::Symbol(Symbol::OpeningParenthesis) => ExpressionWrap::parse_with(
                token,
                lexer,
                errors,
                ("Expected <expr>", Symbol::ClosingParenthesis),
            )
            .map(Pointer::new)
            .map(Self::ExpressionWrap),
            _ => None,
        }
    }
}
impl TryParse for TableAccessPrefix {}

impl Parse for TableAccessKey {
    fn parse(maybe_dot: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        Self::parse_with(maybe_dot, lexer, errors, true)
    }
}
impl TryParse for TableAccessKey {}

impl ParseWithArgs<bool> for TableAccessKey {
    fn parse_with(
        maybe_dot: Token,
        lexer: &mut Lexer,
        errors: &mut Vec<ParseError>,
        accept_expression: bool,
    ) -> Option<Self> {
        match maybe_dot.token_type {
            TokenType::Error(error) => handle_error_token!(errors, error),
            TokenType::Symbol(Symbol::Dot) => {
                next_token_recoverable!(
                    lexer,
                    name,
                    TokenType::Identifier(_) | TokenType::PartialKeyword(_),
                    TokenType::Identifier("*error*".into()),
                    errors,
                    "Expected <ident>"
                );

                Some(Self::Name {
                    dot: Pointer::new(maybe_dot),
                    name: Pointer::new(name),
                })
            }
            _ if accept_expression => TableKey::parse_with(maybe_dot, lexer, errors, false)
                .map(Pointer::new)
                .map(Self::Expression),
            _ => None,
        }
    }
}

impl Parse for TableAccess {
    fn parse(token: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        Some(Self {
            prefix: TableAccessPrefix::parse(token, lexer, errors)?,
            accessed_keys: Vec::<TableAccessKey>::try_parse(lexer, errors)?,
        })
    }
}
impl TryParse for TableAccess {
    fn try_parse(lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        Some(Self {
            prefix: TableAccessPrefix::try_parse(lexer, errors)?,
            accessed_keys: Vec::<TableAccessKey>::try_parse(lexer, errors)?,
        })
    }
}
