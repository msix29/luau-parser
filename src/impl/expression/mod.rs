mod function;
mod table;
mod var;

use luau_lexer::prelude::{Lexer, Literal, ParseError, Symbol, Token, TokenType};

use crate::{
    handle_error_token,
    types::{Expression, ExpressionWrap, FunctionCall, Parse, ParseWithArgs, PrefixExp, Var},
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
}

impl Parse for Expression {
    fn parse(token: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        match token.token_type {
            TokenType::Error(error) => handle_error_token!(errors, error),
            TokenType::Literal(_) => Self::parse_from_literal(token),
            TokenType::Identifier(_) | TokenType::PartialKeyword(_) => Var::parse(token, lexer, errors),
            TokenType::Keyword(_) => todo!(),
            TokenType::Symbol(Symbol::OpeningParenthesis) => ExpressionWrap::parse_with(
                token,
                lexer,
                errors,
                ("Expected <expr>", Symbol::ClosingParenthesis),
            )
            .map(Self::ExpressionWrap),
            TokenType::Symbol(_) => todo!(),
            TokenType::Operator(_) => todo!(),
            TokenType::CompoundOperator(_) => todo!(),
            TokenType::EndOfFile => None,
        }
    }
}
