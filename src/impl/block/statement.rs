use luau_lexer::prelude::{Keyword, Lexer, ParseError, PartialKeyword, Token, TokenType};

use crate::{
    handle_error_token,
    types::{Expression, List, Parse, Pointer, Statement, TerminationStatement},
};

impl Parse for Statement {
    fn parse(token: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        match token.token_type {
            TokenType::Error(error) => handle_error_token!(errors, error),
            TokenType::EndOfFile => None,
            _ => Self::__parse(token, lexer, errors),
        }
    }
}

impl Parse for TerminationStatement {
    fn parse(keyword: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        if !matches!(
            keyword.token_type,
            TokenType::Keyword(Keyword::Break)
                | TokenType::Keyword(Keyword::Return)
                | TokenType::PartialKeyword(PartialKeyword::Continue)
        ) {
            return None;
        }
        if matches!(keyword.token_type, TokenType::Keyword(Keyword::Break)) {
            return Some(Self::Break(keyword));
        }

        if matches!(
            keyword.token_type,
            TokenType::PartialKeyword(PartialKeyword::Continue)
        ) {
            return Some(Self::Continue(keyword));
        }

        if matches!(keyword.token_type, TokenType::Keyword(Keyword::Return)) {
            return Some(Self::Return {
                return_keyword: keyword,
                expressions: List::<Pointer<Expression>>::parse(lexer.next_token(), lexer, errors),
            });
        }

        None
    }
}
