mod function;
mod table;
mod var;

use luau_lexer::prelude::{
    CompoundOperator, Keyword, Lexer, Literal, Operator, ParseError, Symbol, Token, TokenType,
};

use crate::{
    handle_error_token, safe_unwrap,
    types::{
        Closure, ElseIfExpression, Expression, ExpressionWrap, FunctionCall, IfExpression, Parse,
        ParseWithArgs, Pointer, PrefixExp, Table, TypeValue, Var,
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
            TokenType::Keyword(Keyword::Nil) => Some(Self::Nil(token)),
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
                expression: Pointer::new(left),
            }
        } else {
            left
        };

        let state = lexer.save_state();
        let next_token = lexer.next_token();

        match next_token.token_type {
            TokenType::Operator(_) | TokenType::CompoundOperator(CompoundOperator::EqualEqual) => {
                Some(Self::BinaryExpression {
                    left: Pointer::new(left),
                    operator: next_token,
                    right: safe_unwrap!(
                        lexer,
                        errors,
                        "Expected <expr>",
                        Self::parse(lexer.next_token(), lexer, errors).map(Pointer::new)
                    ),
                })
            }
            TokenType::Symbol(Symbol::Typecast) => Some(Self::TypeCast {
                expression: Pointer::new(left),
                operator: next_token,
                cast_to: safe_unwrap!(
                    lexer,
                    errors,
                    "Expected <type>",
                    TypeValue::parse(lexer.next_token(), lexer, errors).map(Pointer::new)
                ),
            }),
            _ => {
                lexer.set_state(state);
                Some(left)
            }
        }
    }
}

impl Parse for IfExpression {
    fn parse(if_keyword: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        if if_keyword != TokenType::Keyword(Keyword::If) {
            return None;
        }

        let condition = safe_unwrap!(
            lexer,
            errors,
            "Expected <expr>",
            try_parse::<Expression>(lexer.save_state(), lexer.next_token(), lexer, errors)
                .map(Pointer::new)
        );

        next_token_recoverable!(
            lexer,
            then_keyword,
            TokenType::Keyword(Keyword::Then),
            TokenType::Keyword(Keyword::Then),
            errors,
            "Expected `then`"
        );

        let if_expression = safe_unwrap!(
            lexer,
            errors,
            "Expected <expr>",
            try_parse::<Expression>(lexer.save_state(), lexer.next_token(), lexer, errors)
                .map(Pointer::new)
        );

        let else_if_expressions =
            try_parse::<Vec<_>>(lexer.save_state(), lexer.next_token(), lexer, errors)
                .map(Pointer::new)
                .unwrap_or_default();

        next_token_recoverable!(
            lexer,
            else_keyword,
            TokenType::Keyword(Keyword::Else),
            TokenType::Keyword(Keyword::Else),
            errors,
            "Expected `else`"
        );
        let else_expression = safe_unwrap!(
            lexer,
            errors,
            "Expected <expr>",
            Expression::parse(lexer.next_token(), lexer, errors).map(Pointer::new)
        );

        Some(Self {
            if_keyword,
            condition,
            then_keyword,
            if_expression,
            else_if_expressions,
            else_keyword,
            else_expression,
        })
    }
}

impl Parse for ElseIfExpression {
    fn parse(
        else_if_keyword: Token,
        lexer: &mut Lexer,
        errors: &mut Vec<ParseError>,
    ) -> Option<Self> {
        if else_if_keyword != TokenType::Keyword(Keyword::Elseif) {
            return None;
        }

        let condition = safe_unwrap!(
            lexer,
            errors,
            "Expected <expr>",
            try_parse::<Expression>(lexer.save_state(), lexer.next_token(), lexer, errors)
                .map(Pointer::new)
        );

        next_token_recoverable!(
            lexer,
            then_keyword,
            TokenType::Keyword(Keyword::Then),
            TokenType::Keyword(Keyword::Then),
            errors,
            "Expected `then`"
        );

        let expression = safe_unwrap!(
            lexer,
            errors,
            "Expected <expr>",
            try_parse::<Expression>(lexer.save_state(), lexer.next_token(), lexer, errors)
                .map(Pointer::new)
        );

        Some(Self {
            else_if_keyword,
            condition,
            then_keyword,
            expression,
        })
    }
}
