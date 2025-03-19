mod function;
mod table;
mod var;

use luau_lexer::prelude::{Lexer, Literal, ParseError, Symbol, Token, TokenType};
use std::sync::Arc;

use crate::types::{Expression, ExpressionWrap, FunctionCall, Parse, PrefixExp, Var};

impl Parse for ExpressionWrap {
    fn parse(
        opening_parenthesis: Token,
        lexer: &mut Lexer,
        errors: &mut Vec<ParseError>,
    ) -> Option<Self> {
        let Some(expression) = Expression::parse(lexer.next_token(), lexer, errors).map(Arc::new)
        else {
            let state = lexer.save_state();
            errors.push(ParseError::new(
                state.lexer_position(),
                "Expected <expr>".to_string(),
                Some(state.lexer_position()),
            ));

            return None;
        };

        next_token_recoverable!(
            lexer,
            closing_parenthesis,
            TokenType::Symbol(Symbol::ClosingParenthesis),
            TokenType::Symbol(Symbol::ClosingParenthesis),
            errors,
            "Expected <opening-parenthesis>"
        );

        Some(Self {
            opening_parenthesis,
            expression,
            closing_parenthesis,
        })
    }
}

impl Parse for PrefixExp {
    fn parse(token: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        Var::parse(token.clone(), lexer, errors)
            .map(Self::Var)
            .or_else(|| FunctionCall::parse(token.clone(), lexer, errors).map(Self::FunctionCall))
            .or_else(|| ExpressionWrap::parse(token, lexer, errors).map(Self::ExpressionWrap))
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
}

impl Parse for Expression {
    fn parse(token: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        match token.token_type {
            TokenType::Error(parse_error) => {
                errors.push(parse_error);

                None
            }
            TokenType::Literal(_) => Self::parse_from_literal(token),
            TokenType::Identifier(_) => Var::parse(token, lexer, errors).map(Self::Var),
            TokenType::Keyword(_) => todo!(),
            TokenType::PartialKeyword(_) => Var::parse(token, lexer, errors).map(Self::Var),
            TokenType::Symbol(Symbol::OpeningParenthesis) => {
                ExpressionWrap::parse(token, lexer, errors).map(Self::ExpressionWrap)
            }
            TokenType::Symbol(_) => todo!(),
            TokenType::Operator(_) => todo!(),
            TokenType::CompoundOperator(_) => todo!(),
            TokenType::EndOfFile => None,
        }
    }
}
