//! All `impl` blocks for [`Statement`] and [`TerminationStatement`].

use luau_lexer::prelude::{Keyword, Lexer, ParseError, PartialKeyword, Token, TokenType};

use crate::{
    handle_error_token,
    types::{Expression, List, Parse, Pointer, Print, Statement, TerminationStatement, TryParse},
};

impl Parse for Statement {
    fn parse(token: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        match token.token_type {
            TokenType::Error(error) => handle_error_token!(errors, error),
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
            let state = lexer.save_state();
            let mut expressions = List::<Pointer<Expression>>::try_parse(lexer, errors);
            if expressions
                .as_ref()
                .map(|expressions| expressions.is_empty())
                .unwrap_or(false)
            {
                expressions = None;
                lexer.set_state(state);
            }

            return Some(Self::Return {
                return_keyword: keyword,
                expressions,
            });
        }

        None
    }
}

impl Print for TerminationStatement {
    fn print_final_trivia(&self) -> String {
        match self {
            TerminationStatement::Break(token) | TerminationStatement::Continue(token) => {
                token.print_final_trivia()
            }
            TerminationStatement::Return {
                return_keyword,
                expressions,
            } => {
                if let Some(expressions) = expressions {
                    expressions.print_final_trivia()
                } else {
                    return_keyword.print_final_trivia()
                }
            }
        }
    }

    fn print_without_final_trivia(&self) -> String {
        match self {
            TerminationStatement::Break(token) | TerminationStatement::Continue(token) => {
                token.print_without_final_trivia()
            }
            TerminationStatement::Return {
                return_keyword,
                expressions,
            } => {
                return_keyword.print_without_final_trivia()
                    + &expressions.print_without_final_trivia()
            }
        }
    }
}
