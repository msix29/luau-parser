mod function;
mod table;
mod var;

use std::sync::Arc;

use luau_lexer::{
    prelude::{Lexer, Literal, ParseError, Symbol, Token, TokenType},
    token::{Keyword, Operator},
};

use crate::{
    handle_error_token,
    types::{
        Closure, ElseIfExpression, Expression, ExpressionWrap, FunctionCall, IfExpression, Parse,
        ParseWithArgs, PrefixExp, Table, TypeValue, Var,
    },
    utils::try_parse,
};

impl Parse for PrefixExp {
    #[inline]
    fn parse(token: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        Self::parse_with(token, lexer, errors, false)
    }
}

impl ParseWithArgs<bool> for PrefixExp {
    fn parse_with(
        token: Token,
        lexer: &mut Lexer,
        errors: &mut Vec<ParseError>,
        is_recursion: bool,
    ) -> Option<Self> {
        (!is_recursion)
            .then(|| FunctionCall::parse(token.clone(), lexer, errors))
            .unwrap_or(None)
            .or_else(|| Var::parse_with(token.clone(), lexer, errors, is_recursion))
            .or_else(|| {
                ExpressionWrap::parse_with(
                    token,
                    lexer,
                    errors,
                    ("Expected <expr>", Symbol::ClosingParenthesis),
                )
                .map(Self::ExpressionWrap)
            })
    }
}

impl Expression {
    pub fn parse_from_literal(token: Token) -> Option<Self> {
        match &token.token_type {
            TokenType::Literal(literal) => match literal {
                Literal::Number(_) => Some(Self::Number(token)),
                Literal::String(_) => Some(Self::String(token)),
                Literal::Boolean(_) => Some(Self::Boolean(token)),
            },
            _ => None,
        }
    }

    fn parse_inner(token: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        match token.token_type {
            TokenType::Error(error) => handle_error_token!(errors, error),
            TokenType::Literal(_) => Self::parse_from_literal(token),
            TokenType::Identifier(_) | TokenType::PartialKeyword(_) => {
                Var::parse(token, lexer, errors)
            }
            TokenType::Symbol(Symbol::OpeningParenthesis) => ExpressionWrap::parse_with(
                token,
                lexer,
                errors,
                ("Expected <expr>", Symbol::ClosingParenthesis),
            )
            .map(Self::ExpressionWrap),
            TokenType::Symbol(Symbol::OpeningCurlyBrackets) => {
                Table::parse_with(token, lexer, errors, false).map(Self::Table)
            }
            TokenType::Keyword(Keyword::Function) => {
                Closure::parse(token, lexer, errors).map(Self::Closure)
            }
            TokenType::Keyword(Keyword::If) => {
                IfExpression::parse(token, lexer, errors).map(Self::IfExpression)
            }
            _ => None,
        }
    }
}

impl Parse for Expression {
    fn parse(mut token: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        let maybe_unary_operator = match token.token_type {
            TokenType::Operator(Operator::Minus | Operator::Not) => {
                let temp = token;
                token = lexer.next_token();

                Some(temp)
            }
            _ => None,
        };

        let left = Self::parse_inner(token, lexer, errors)?;
        let left = if let Some(operator) = maybe_unary_operator {
            Self::UnaryExpression {
                operator,
                expression: Arc::new(left),
            }
        } else {
            left
        };

        let state = lexer.save_state();
        let next_token = lexer.next_token();

        match next_token.token_type {
            TokenType::Operator(_) => Some(Self::BinaryExpression {
                left: Arc::new(left),
                operator: next_token,
                right: Self::parse(lexer.next_token(), lexer, errors)
                    .map(Arc::new)
                    .unwrap_or_default(),
            }),
            TokenType::Symbol(Symbol::Typecast) => Some(Self::TypeCast {
                expression: Arc::new(left),
                operator: next_token,
                cast_to: TypeValue::parse(lexer.next_token(), lexer, errors)
                    .map(Arc::new)
                    .unwrap_or_default(),
            }),
            _ => {
                lexer.set_state(state);
                Some(left)
            }
        }
    }
}

impl Parse for IfExpression {
    fn parse(if_token: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        if if_token != TokenType::Keyword(Keyword::If) {
            return None;
        }

        let condition =
            try_parse::<Expression>(lexer.save_state(), lexer.next_token(), lexer, errors)
                .map(Arc::new)
                .unwrap_or_default();

        next_token_recoverable!(
            lexer,
            then_token,
            TokenType::Keyword(Keyword::Then),
            TokenType::Keyword(Keyword::Then),
            errors,
            "Expected `then`"
        );

        let if_expression =
            try_parse::<Expression>(lexer.save_state(), lexer.next_token(), lexer, errors)
                .map(Arc::new)
                .unwrap_or_default();

        let else_if_expressions =
            try_parse::<Vec<_>>(lexer.save_state(), lexer.next_token(), lexer, errors)
                .map(Arc::new)
                .unwrap_or_default();

        next_token_recoverable!(
            lexer,
            else_token,
            TokenType::Keyword(Keyword::Else),
            TokenType::Keyword(Keyword::Else),
            errors,
            "Expected `else`"
        );
        let else_expression = Expression::parse(lexer.next_token(), lexer, errors)
            .map(Arc::new)
            .unwrap_or_default();

        Some(Self {
            if_token,
            condition,
            then_token,
            if_expression,
            else_if_expressions,
            else_token,
            else_expression,
        })
    }
}

impl Parse for ElseIfExpression {
    fn parse(
        else_if_token: Token,
        lexer: &mut Lexer,
        errors: &mut Vec<ParseError>,
    ) -> Option<Self> {
        if else_if_token != TokenType::Keyword(Keyword::Elseif) {
            return None;
        }

        let condition =
            try_parse::<Expression>(lexer.save_state(), lexer.next_token(), lexer, errors)
                .map(Arc::new)
                .unwrap_or_default();

        next_token_recoverable!(
            lexer,
            then_token,
            TokenType::Keyword(Keyword::Then),
            TokenType::Keyword(Keyword::Then),
            errors,
            "Expected `then`"
        );

        let expression =
            try_parse::<Expression>(lexer.save_state(), lexer.next_token(), lexer, errors)
                .map(Arc::new)
                .unwrap_or_default();

        Some(Self {
            else_if_token,
            condition,
            then_token,
            expression,
        })
    }
}
