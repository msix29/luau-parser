use std::sync::Arc;

use luau_lexer::{
    prelude::{Lexer, ParseError, Token, TokenType},
    token::Symbol,
};

use crate::{
    types::{Block, Parse, ParseWithArgs, Statement, TerminationStatement},
    utils::get_token_type_display_extended,
};

impl ParseWithArgs<Option<TokenType>> for Block {
    fn parse_with(
        mut token: Token,
        lexer: &mut Lexer,
        errors: &mut Vec<ParseError>,
        stop_at: Option<TokenType>,
    ) -> Option<Self> {
        let stop_at = stop_at.as_ref();
        let mut statements = Vec::new();
        let mut last_statement = None;

        loop {
            if token.token_type == TokenType::EndOfFile {
                break;
            }
            let mut failed_parsing = false;

            if let Some(statement) = Statement::parse(token.clone(), lexer, errors) {
                if last_statement.is_some() {
                    // We will still continue parsing so LSPs, formatters, etc.
                    // can still produce "correct" outputs.

                    //TODO:
                    // if let Some(location) = statement.get_location() {
                    //     errors.push(
                    //         ParseError::new(
                    //             location.start,
                    //             "Statements after a termination statement are not allowed."
                    //                 .to_string(),
                    //             Some(location.end),
                    //         )
                    //         .into(),
                    //     );
                    // }
                }

                maybe_next_token!(lexer, maybe_semicolon, TokenType::Symbol(Symbol::Semicolon));
                statements.push((Arc::new(statement), maybe_semicolon))
            } else if let Some(statement) =
                TerminationStatement::parse(token.clone(), lexer, errors)
            {
                maybe_next_token!(lexer, maybe_semicolon, TokenType::Symbol(Symbol::Semicolon));
                last_statement = Some((Arc::new(statement), maybe_semicolon));
            } else {
                failed_parsing = true;
            }

            let state = lexer.save_state();
            let next_token = lexer.next_token();

            if Some(&next_token.token_type) == stop_at {
                lexer.set_state(state);

                break;
            } else if token.token_type != TokenType::EndOfFile && failed_parsing {
                errors.push(ParseError::new(
                    state.lexer_position(),
                    format!(
                        "Unexpected {}",
                        get_token_type_display_extended(&token.token_type)
                    ),
                    Some(state.lexer_position()),
                ));
            }

            token = next_token;
        }

        Some(Self {
            statements,
            last_statement,
        })
    }
}

impl Block {
    pub fn is_empty(&self) -> bool {
        self.statements.is_empty() && self.last_statement.is_none()
    }
}
