//! All `impl` blocks for [`LocalAssignment`].

use luau_lexer::prelude::{Keyword, Lexer, ParseError, Symbol, Token, TokenType};

use crate::types::{Expression, List, LocalAssignment, Name, Parse, Pointer, TryParse};

impl Parse for LocalAssignment {
    fn parse(local_token: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        if local_token != TokenType::Keyword(Keyword::Local) {
            return None;
        }

        let name_list = List::<Name>::try_parse(lexer, errors)?;

        maybe_next_token!(lexer, equal_token, TokenType::Symbol(Symbol::Equal));
        let expressions = if equal_token.is_some() {
            List::<Pointer<Expression>>::try_parse(lexer, errors)?
        } else {
            List::<Pointer<Expression>>::new()
        };

        Some(Self {
            local_token,
            name_list,
            equal_token,
            expressions,
        })
    }
}
