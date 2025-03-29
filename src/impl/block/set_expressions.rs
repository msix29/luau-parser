use luau_lexer::prelude::{CompoundOperator, Lexer, ParseError, Symbol, Token, TokenType};

use crate::{
    safe_unwrap,
    types::{
        CompoundSetExpression, Expression, List, Parse, Pointer, SetExpression, TryParse, Var,
    },
    utils::{get_token_type_display, get_token_type_display_extended},
};

impl Parse for SetExpression {
    fn parse(token: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        if !matches!(
            token.token_type,
            TokenType::PartialKeyword(_) | TokenType::Identifier(_)
        ) {
            return None;
        }

        let variables = safe_unwrap!(
            lexer,
            errors,
            "Expected <name>",
            List::parse(token, lexer, errors)
        );
        next_token_recoverable!(
            lexer,
            equal,
            TokenType::Symbol(Symbol::Equal),
            TokenType::Symbol(Symbol::Equal),
            errors,
            "Expected ".to_string()
                + get_token_type_display_extended(&TokenType::Symbol(Symbol::Equal))
        );
        let values = safe_unwrap!(
            lexer,
            errors,
            "Expected <expr>",
            List::try_parse(lexer, errors)
        );

        Some(Self {
            variables,
            equal,
            values,
        })
    }
}

impl Parse for CompoundSetExpression {
    fn parse(token: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        if !matches!(
            token.token_type,
            TokenType::PartialKeyword(_) | TokenType::Identifier(_)
        ) {
            return None;
        }

        let variable = safe_unwrap!(
            lexer,
            errors,
            "Expected <name>",
            Var::parse(token, lexer, errors)
        );
        next_token_recoverable!(
            lexer,
            operation,
            TokenType::CompoundOperator(CompoundOperator::PlusEqual),
            TokenType::CompoundOperator(CompoundOperator::PlusEqual),
            errors,
            "Expected ".to_string()
                + get_token_type_display(&TokenType::CompoundOperator(CompoundOperator::PlusEqual))
        );
        let value = safe_unwrap!(
            lexer,
            errors,
            "Expected <expr>",
            Pointer::<Expression>::try_parse(lexer, errors)
        );

        Some(Self {
            variable,
            operation,
            value,
        })
    }
}
