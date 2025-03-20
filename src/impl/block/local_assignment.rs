use std::sync::Arc;
use luau_lexer::prelude::{Lexer, Operator, ParseError, Token, TokenType};

use crate::types::{Expression, List, LocalAssignment, Name, Parse};

impl Parse for LocalAssignment {
    fn parse(local_token: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        let name_list = List::<Name>::parse(lexer.next_token(), lexer, errors)?;

        maybe_next_token!(lexer, equal_token, TokenType::Operator(Operator::Equal));
        let expressions = if equal_token.is_some() {
            List::<Arc<Expression>>::new()
        } else {
            List::<Arc<Expression>>::parse(lexer.next_token(), lexer, errors)?
        };

        Some(Self {
            local_token,
            name_list,
            equal_token,
            expressions,
        })
    }
}
