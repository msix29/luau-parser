//! All `impl` blocks for [`Var`].

use luau_lexer::prelude::{Lexer, ParseError, Symbol, Token, TokenType};

use crate::{
    types::{
        Expression, FunctionCall, FunctionCallInvoked, Parse, Pointer, PrefixExp, TableAccess,
        TableAccessKey, TableAccessPrefix, TryParse, Var,
    },
    utils::get_token_type_display,
};

impl Parse for Var {
    fn parse(token: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        if !matches!(
            token.token_type,
            TokenType::PartialKeyword(_) | TokenType::Identifier(_)
        ) {
            return None;
        }

        maybe_next_token!(
            lexer,
            maybe_dot_or_bracket,
            TokenType::Symbol(Symbol::Dot) | TokenType::Symbol(Symbol::OpeningBrackets)
        );
        if let Some(dot_or_bracket) = maybe_dot_or_bracket {
            return Some(Self::TableAccess(TableAccess {
                prefix: TableAccessPrefix::Name(token),
                accessed_keys: Vec::<TableAccessKey>::parse(dot_or_bracket, lexer, errors)?,
            }));
        }

        Some(Self::Name(token))
    }
}
impl TryParse for Var {}
