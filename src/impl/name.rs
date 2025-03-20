use std::sync::Arc;

use luau_lexer::prelude::{Lexer, ParseError, Symbol, Token, TokenType};

use crate::types::{Name, Parse, TypeValue};

impl Parse for Name {
    fn parse(name: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        if !matches!(
            name.token_type,
            TokenType::Identifier(_) | TokenType::PartialKeyword(_)
        ) {
            return None;
        }

        maybe_next_token!(lexer, colon, TokenType::Symbol(Symbol::Colon));

        let r#type = if colon.is_some() {
            TypeValue::parse(lexer.next_token(), lexer, errors).map(Arc::new)
        } else {
            None
        };

        Some(Self {
            name,
            colon,
            r#type,
        })
    }
}
