use luau_lexer::{
    prelude::{Lexer, ParseError, Token, TokenType},
    token::Symbol,
};

use crate::types::{
    Expression, Parse, PrefixExp, TableAccess, TableAccessKey, TableAccessPrefix, TryParse, Var,
};

impl Parse for Var {
    fn parse(token: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        if !matches!(
            token.token_type,
            TokenType::PartialKeyword(_) | TokenType::Identifier(_)
        ) {
            return None;
        }

        maybe_next_token!(lexer, maybe_dot, TokenType::Symbol(Symbol::Dot));
        if let Some(dot) = maybe_dot {
            return Some(Self::TableAccess(TableAccess {
                prefix: TableAccessPrefix::Name(token),
                accessed_keys: Vec::<TableAccessKey>::parse(dot, lexer, errors)?,
            }));
        }

        Some(Self::Name(token))
    }
}
impl TryParse for Var {}

impl Parse<PrefixExp> for Var {
    #[inline]
    fn parse(token: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<PrefixExp> {
        Self::parse(token, lexer, errors).map(PrefixExp::Var)
    }
}
impl TryParse<PrefixExp> for Var {}

impl Parse<Expression> for Var {
    #[inline]
    fn parse(token: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Expression> {
        Self::parse(token, lexer, errors).map(Expression::Var)
    }
}
impl TryParse<Expression> for Var {}
