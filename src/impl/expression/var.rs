use luau_lexer::prelude::{Lexer, ParseError, Token, TokenType};

use crate::types::{Parse, TableAccess, Var};

impl Parse for Var {
    fn parse(token: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        if !matches!(
            token.token_type,
            TokenType::PartialKeyword(_) | TokenType::Identifier(_)
        ) {
            return None;
        }

        let state = lexer.save_state();
        if let Some(table_access) = TableAccess::parse(token.clone(), lexer, errors) {
            return Some(Self::TableAccess(table_access));
        }

        // `TableAccess::parse` might match the prefix but not the accessed keys
        // so we need to return the state back to it's original.
        lexer.set_state(state);

        Some(Self::Name(token))
    }
}
