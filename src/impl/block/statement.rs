use luau_lexer::prelude::{Keyword, Lexer, ParseError, PartialKeyword, Symbol, Token, TokenType};

use crate::types::{List, Parse, Statement, TerminationStatement};

impl Parse for Statement {
    fn parse(token: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        todo!()
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
            maybe_next_token!(lexer, semi_colon, TokenType::Symbol(Symbol::Semicolon));

            return Some(Self::Break((keyword, semi_colon)));
        }

        if matches!(
            keyword.token_type,
            TokenType::PartialKeyword(PartialKeyword::Continue)
        ) {
            maybe_next_token!(lexer, semi_colon, TokenType::Symbol(Symbol::Semicolon));

            return Some(Self::Continue((keyword, semi_colon)));
        }

        if matches!(keyword.token_type, TokenType::Keyword(Keyword::Return)) {
            let expressions = List::new(); //TODO

            maybe_next_token!(lexer, semi_colon, TokenType::Symbol(Symbol::Semicolon));

            return Some(Self::Return {
                return_keyword: keyword,
                expressions,
                semicolon: semi_colon,
            });
        }

        None
    }
}
