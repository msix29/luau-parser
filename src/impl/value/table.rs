use luau_lexer::{
    prelude::{Lexer, Operator, ParseError, Symbol, Token, TokenType},
    token::Literal,
};
use std::sync::Arc;

use crate::types::{
    Bracketed, BracketedList, Expression, FunctionArguments, Parse, ParseWithArgs, Table,
    TableAccessKey, TableField, TableFieldValue, TableKey, TypeValue,
};

impl ParseWithArgs<bool> for TableKey {
    fn parse_with(
        token: Token,
        lexer: &mut Lexer,
        errors: &mut Vec<ParseError>,
        is_type: bool,
    ) -> Option<Self> {
        match token.token_type {
            TokenType::Literal(Literal::String(_)) => Some(Self::String(token)),
            _ if lexer.next_token() == TokenType::Symbol(Symbol::OpeningBrackets) => {
                if is_type {
                    Bracketed::<_>::parse_with(
                        token,
                        lexer,
                        errors,
                        ("Expected <type>", Symbol::ClosingBrackets),
                    )
                    .map(Self::Type)
                } else {
                    Bracketed::<_>::parse_with(
                        token,
                        lexer,
                        errors,
                        ("Expected <expr>", Symbol::ClosingBrackets),
                    )
                    .map(Self::Expression)
                }
            }
            _ => None,
        }
    }
}

impl ParseWithArgs<(bool, &mut u32)> for TableField {
    fn parse_with(
        token: Token,
        lexer: &mut Lexer,
        errors: &mut Vec<ParseError>,
        (is_type, inferred_keys): (bool, &mut u32),
    ) -> Option<Self> {
        if let Some(value) = TableFieldValue::parse_with(token.clone(), lexer, errors, is_type) {
            return Some(Self {
                key: Arc::new(TableKey::UndefinedString("number".into())),
                equal_or_colon: None,
                value: Arc::new(value),
            });
        }

        let (key, equal_or_colon) =
            if let Some(key) = TableKey::parse_with(token, lexer, errors, is_type) {
                let equal_or_colon;

                if is_type {
                    next_token_recoverable!(
                        lexer,
                        temp,
                        TokenType::Symbol(Symbol::Colon),
                        TokenType::Symbol(Symbol::Colon),
                        errors,
                        "Expected <colon>"
                    );

                    equal_or_colon = temp;
                } else {
                    next_token_recoverable!(
                        lexer,
                        temp,
                        TokenType::Operator(Operator::Equal),
                        TokenType::Operator(Operator::Equal),
                        errors,
                        "Expected <equal>"
                    );

                    equal_or_colon = temp;
                }

                (Arc::new(key), Some(equal_or_colon))
            } else {
                *inferred_keys += 1;

                (Arc::new(TableKey::UndefinedNumber(*inferred_keys)), None)
            };

        let value = Arc::new(TableFieldValue::parse_with(
            lexer.next_token(),
            lexer,
            errors,
            is_type,
        )?);

        Some(Self {
            key,
            equal_or_colon,
            value,
        })
    }
}

impl ParseWithArgs<bool> for TableFieldValue {
    #[inline]
    fn parse_with(
        token: Token,
        lexer: &mut Lexer,
        errors: &mut Vec<ParseError>,
        is_type: bool,
    ) -> Option<Self> {
        if is_type {
            TypeValue::parse(token, lexer, errors).map(Self::Type)
        } else {
            Expression::parse(token, lexer, errors).map(Self::Expression)
        }
    }
}

impl ParseWithArgs<bool> for Table {
    fn parse_with(
        token: Token,
        lexer: &mut Lexer,
        errors: &mut Vec<ParseError>,
        is_type: bool,
    ) -> Option<Self> {
        if !matches!(
            token.token_type,
            TokenType::Symbol(Symbol::OpeningCurlyBrackets)
        ) {
            return None;
        }

        BracketedList::<TableField>::parse_with(
            token,
            lexer,
            errors,
            (
                "Expected <table-field>",
                Symbol::ClosingCurlyBrackets,
                (is_type, &mut 0),
            ),
        )
        .map(Self)
    }
}

impl Parse<FunctionArguments> for Table {
    #[inline]
    fn parse(
        token: Token,
        lexer: &mut Lexer,
        errors: &mut Vec<ParseError>,
    ) -> Option<FunctionArguments> {
        Self::parse_with(token, lexer, errors, false).map(FunctionArguments::Table)
    }
}

impl Parse<TableAccessKey> for TableKey {
    #[inline]
    fn parse(
        token: Token,
        lexer: &mut Lexer,
        errors: &mut Vec<ParseError>,
    ) -> Option<TableAccessKey> {
        Self::parse_with(token, lexer, errors, false)
            .map(Box::new)
            .map(TableAccessKey::Expression)
    }
}
