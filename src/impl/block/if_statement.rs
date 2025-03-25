use luau_lexer::{
    prelude::{Lexer, ParseError, Token},
    token::{Keyword, TokenType},
};
use std::sync::Arc;

use crate::{
    safe_unwrap,
    types::{Block, ElseIfStatement, ElseStatement, Expression, IfStatement, Parse, ParseWithArgs},
    utils::try_parse,
};

const END_TOKENS: [TokenType; 3] = [
    TokenType::Keyword(Keyword::End),
    TokenType::Keyword(Keyword::Elseif),
    TokenType::Keyword(Keyword::Else),
];

impl Parse for IfStatement {
    fn parse(if_keyword: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        if if_keyword != TokenType::Keyword(Keyword::If) {
            return None;
        }

        let condition = safe_unwrap!(
            lexer,
            errors,
            "Expected <expr>",
            try_parse::<Expression>(lexer.save_state(), lexer.next_token(), lexer, errors)
                .map(Arc::new)
        );

        next_token_recoverable!(
            lexer,
            then_keyword,
            TokenType::Keyword(Keyword::Then),
            TokenType::Keyword(Keyword::Then),
            errors,
            "Expected `then`"
        );

        let body =
            Block::parse_with(lexer.next_token(), lexer, errors, END_TOKENS).unwrap_or_default();

        let else_if_statements =
            try_parse::<Vec<_>>(lexer.save_state(), lexer.next_token(), lexer, errors)
                .unwrap_or_default();

        let else_statement =
            try_parse::<ElseStatement>(lexer.save_state(), lexer.next_token(), lexer, errors);

        next_token_recoverable!(
            lexer,
            end_keyword,
            TokenType::Keyword(Keyword::End),
            TokenType::Keyword(Keyword::End),
            errors,
            "Expected `end`"
        );

        Some(Self {
            if_keyword,
            condition,
            then_keyword,
            body,
            else_if_statements,
            else_statement,
            end_keyword,
        })
    }
}

impl Parse for ElseIfStatement {
    fn parse(
        elseif_keyword: Token,
        lexer: &mut Lexer,
        errors: &mut Vec<ParseError>,
    ) -> Option<Self> {
        if elseif_keyword != TokenType::Keyword(Keyword::Elseif) {
            return None;
        }

        let condition = safe_unwrap!(
            lexer,
            errors,
            "Expected <expr>",
            try_parse::<Expression>(lexer.save_state(), lexer.next_token(), lexer, errors)
                .map(Arc::new)
        );

        next_token_recoverable!(
            lexer,
            then_keyword,
            TokenType::Keyword(Keyword::Then),
            TokenType::Keyword(Keyword::Then),
            errors,
            "Expected `then`"
        );

        let body =
            Block::parse_with(lexer.next_token(), lexer, errors, END_TOKENS).unwrap_or_default();

        Some(Self {
            elseif_keyword,
            condition,
            then_keyword,
            body,
        })
    }
}

impl Parse for ElseStatement {
    fn parse(else_keyword: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        Some(Self {
            else_keyword,
            body: Block::parse_with(
                lexer.next_token(),
                lexer,
                errors,
                TokenType::Keyword(Keyword::End),
            )
            .unwrap_or_default(),
        })
    }
}
