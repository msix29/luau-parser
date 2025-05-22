//! All `impl` blocks for expression-related types

mod function;
mod table;
mod var;

use luau_lexer::prelude::{
    Keyword, Lexer, Literal, Operator, ParseError, Symbol, Token, TokenType,
};

use crate::{
    handle_error_token, safe_unwrap,
    types::{
        Bracketed, Closure, ElseIfExpression, Expression, FunctionCall, FunctionCallInvoked,
        IfExpression, Parse, ParseWithArgs, Pointer, PrefixExp, Table, TableAccess,
        TableAccessPrefix, TryParse, TypeValue, Var,
    },
    utils::get_token_type_display,
};

impl PrefixExp {
    /// Tries parsing more [`PrefixExp`]s starting with this one.
    fn parse_more(&self, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        maybe_next_token!(lexer, colon, TokenType::Symbol(Symbol::Colon));

        let invoked = if let Some(colon) = colon {
            next_token_recoverable!(
                lexer,
                method,
                TokenType::Identifier(_) | TokenType::PartialKeyword(_),
                TokenType::Identifier("*error*".into(),),
                errors,
                "Expected ".to_string()
                    + get_token_type_display(&TokenType::Identifier("".into(),))
            );

            FunctionCallInvoked::TableMethod {
                table: Pointer::new(self.clone()),
                colon: Pointer::new(colon),
                method: Pointer::new(method),
            }
        } else {
            FunctionCallInvoked::Function(Pointer::new(self.clone()))
        };

        if let Some(call) = FunctionCall::try_parse_with_invoked(lexer, errors, invoked) {
            let prefix_exp = Self::FunctionCall(call);

            return prefix_exp.parse_more(lexer, errors).or(Some(prefix_exp));
        }

        if let Some(accessed_keys) = Vec::try_parse(lexer, errors) {
            let prefix_exp = Self::Var(Var::TableAccess(TableAccess {
                prefix: match self {
                    PrefixExp::FunctionCall(function_call) => {
                        TableAccessPrefix::FunctionCall(Pointer::new(function_call.clone()))
                    }
                    PrefixExp::ExpressionWrap(bracketed) => {
                        TableAccessPrefix::ExpressionWrap(Pointer::new(bracketed.clone()))
                    }
                    _ => unreachable!(),
                },
                accessed_keys,
            }));

            return prefix_exp.parse_more(lexer, errors).or(Some(prefix_exp));
        }

        None
    }
}

impl Parse for PrefixExp {
    fn parse(token: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        let var = Var::parse(token.clone(), lexer, errors);
        if let Some(var) = var {
            let prefix_exp = Self::Var(var);

            return prefix_exp.parse_more(lexer, errors).or(Some(prefix_exp));
        }

        if token == TokenType::Symbol(Symbol::OpeningParenthesis) {
            let expression_wrap = Bracketed::<Pointer<Expression>>::parse_with(
                token,
                lexer,
                errors,
                ("Expected <expr>", Symbol::ClosingParenthesis),
            );

            if let Some(expression_wrap) = expression_wrap {
                let prefix_exp = Self::ExpressionWrap(expression_wrap);

                prefix_exp.parse_more(lexer, errors).or(Some(prefix_exp))
            } else {
                None
            }
        } else {
            None
        }
    }
}
impl TryParse for PrefixExp {}

impl Expression {
    /// Parses an [`Expression`] from a literal [`TokenType::Literal`]
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

    /// Inner function for [`Expression::parse`]. This function doesn't check for
    /// operators before nor after the expression, which [`Expression::parse`] does.
    fn parse_inner(token: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        match token.token_type {
            TokenType::Error(error) => handle_error_token!(errors, error),
            TokenType::Literal(_) => Self::parse_from_literal(token),
            TokenType::Identifier(_)
            | TokenType::PartialKeyword(_)
            | TokenType::Symbol(Symbol::OpeningParenthesis) => {
                match PrefixExp::parse(token, lexer, errors) {
                    Some(PrefixExp::ExpressionWrap(wrap)) => Some(Self::ExpressionWrap(wrap)),
                    Some(PrefixExp::FunctionCall(function_call)) => {
                        Some(Self::FunctionCall(function_call))
                    }
                    Some(PrefixExp::Var(var)) => Some(Self::Var(var)),
                    None => None,
                }
            }
            TokenType::Symbol(Symbol::OpeningCurlyBrackets) => {
                Table::parse_with(token, lexer, errors, false).map(Self::Table)
            }
            TokenType::Keyword(Keyword::Function) | TokenType::Symbol(Symbol::At) => {
                Closure::parse(token, lexer, errors)
                    .map(Pointer::new)
                    .map(Self::Closure)
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
            TokenType::Operator(Operator::Minus | Operator::Not | Operator::Length) => {
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
            TokenType::Operator(_)
            | TokenType::CompoundOperator(_)
            | TokenType::Symbol(Symbol::OpeningAngleBrackets)
            | TokenType::Symbol(Symbol::ClosingAngleBrackets) => Some(Self::BinaryExpression {
                left: Pointer::new(left),
                operator: next_token,
                right: safe_unwrap!(
                    lexer,
                    errors,
                    "Expected <expr>",
                    Self::try_parse(lexer, errors).map(Pointer::new)
                ),
            }),
            TokenType::Symbol(Symbol::Typecast) => Some(Self::TypeCast {
                expression: Pointer::new(left),
                operator: next_token,
                cast_to: safe_unwrap!(
                    lexer,
                    errors,
                    "Expected <type>",
                    TypeValue::try_parse(lexer, errors).map(Pointer::new)
                ),
            }),
            _ => {
                lexer.set_state(state);
                Some(left)
            }
        }
    }
}
impl TryParse for Expression {}

impl Parse for IfExpression {
    fn parse(if_keyword: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        if if_keyword != TokenType::Keyword(Keyword::If) {
            return None;
        }

        let condition = safe_unwrap!(
            lexer,
            errors,
            "Expected <expr>",
            Pointer::<Expression>::try_parse(lexer, errors)
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
            Pointer::<Expression>::try_parse(lexer, errors)
        );

        let else_if_expressions =
            Pointer::<Vec<ElseIfExpression>>::try_parse(lexer, errors).unwrap_or_default();

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
            Expression::try_parse(lexer, errors).map(Pointer::new)
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
impl TryParse for IfExpression {}

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
            Pointer::<Expression>::try_parse(lexer, errors)
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
            Pointer::<Expression>::try_parse(lexer, errors)
        );

        Some(Self {
            else_if_keyword,
            condition,
            then_keyword,
            expression,
        })
    }
}
impl TryParse for ElseIfExpression {}
