use luau_lexer::prelude::{Lexer, ParseError, Symbol, Token, TokenType};

use crate::{
    safe_unwrap,
    types::{
        CompoundSetExpression, Expression, List, Parse, Pointer, SetExpression, TryParse, Var,
    },
};

impl Parse for SetExpression {
    fn parse(token: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        let state = lexer.save_state();

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
        maybe_next_token!(lexer, equal, TokenType::Symbol(Symbol::Equal));
        let Some(equal) = equal else {
            lexer.set_state(state);

            return None;
        };

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
        let state = lexer.save_state();
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
        maybe_next_token!(lexer, operation, TokenType::CompoundOperator(_));
        let Some(operation) = operation else {
            lexer.set_state(state);

            return None;
        };

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
