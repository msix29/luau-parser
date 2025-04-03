use luau_lexer::{
    prelude::{Lexer, ParseError, Token, TokenType},
    token::Symbol,
};

use crate::types::{
    Expression, Parse, ParseWithArgs, PrefixExp, TableAccess, TableAccessKey, TableAccessPrefix, TryParse, TryParseWithArgs, Var
};

impl Parse for Var {
    fn parse(token: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        Self::parse_with(token, lexer, errors, false)
    }
}
impl TryParse for Var {}

impl ParseWithArgs<bool> for Var {
    fn parse_with(
        token: Token,
        lexer: &mut Lexer,
        errors: &mut Vec<ParseError>,
        is_recursion: bool,
    ) -> Option<Self> {
        if !matches!(
            token.token_type,
            TokenType::PartialKeyword(_) | TokenType::Identifier(_)
        ) {
            return None;
        }

        if !is_recursion {
            let state = lexer.save_state();
            if let Some(table_access) = TableAccess::parse(token.clone(), lexer, errors) {
                return Some(Self::TableAccess(table_access));
            }

            // `TableAccess::parse` might match the prefix but not the accessed keys
            // so we need to return the state back to it's original.
            lexer.set_state(state);
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

impl Parse<PrefixExp> for Var {
    #[inline]
    fn parse(token: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<PrefixExp> {
        Self::parse(token, lexer, errors).map(PrefixExp::Var)
    }
}
impl TryParse<PrefixExp> for Var {}

impl ParseWithArgs<bool, PrefixExp> for Var {
    #[inline]
    fn parse_with(
        token: Token,
        lexer: &mut Lexer,
        errors: &mut Vec<ParseError>,
        is_recursion: bool,
    ) -> Option<PrefixExp> {
        Self::parse_with(token, lexer, errors, is_recursion).map(PrefixExp::Var)
    }
}
impl TryParseWithArgs<bool, PrefixExp> for Var {}

impl Parse<Expression> for Var {
    #[inline]
    fn parse(token: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Expression> {
        Self::parse(token, lexer, errors).map(Expression::Var)
    }
}
impl TryParse<Expression> for Var {}

impl ParseWithArgs<bool, Expression> for Var {
    #[inline]
    fn parse_with(
        token: Token,
        lexer: &mut Lexer,
        errors: &mut Vec<ParseError>,
        is_recursion: bool,
    ) -> Option<Expression> {
        Self::parse_with(token, lexer, errors, is_recursion).map(Expression::Var)
    }
}
impl TryParseWithArgs<bool, Expression, Var> for Var {}
