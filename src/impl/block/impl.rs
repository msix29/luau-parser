use luau_lexer::prelude::{Lexer, ParseError, Symbol, Token, TokenType};

use crate::{
    types::{
        Block, GetRange, GetRangeError, Parse, ParseWithArgs, Pointer, Print, Range, Statement,
        TerminationStatement,
    },
    utils::get_token_type_display_extended,
};

trait MatchesToken {
    fn matches(&self, token: &Token) -> bool;
}

impl<T: MatchesToken> MatchesToken for Option<T> {
    #[inline]
    fn matches(&self, token: &Token) -> bool {
        match self {
            Some(value) => value.matches(token),
            None => false,
        }
    }
}

impl MatchesToken for TokenType {
    #[inline]
    fn matches(&self, token: &Token) -> bool {
        token == self
    }
}
impl MatchesToken for Token {
    #[inline]
    fn matches(&self, token: &Token) -> bool {
        token == self
    }
}

impl MatchesToken for Vec<TokenType> {
    #[inline]
    fn matches(&self, token: &Token) -> bool {
        self.contains(&token.token_type)
    }
}
impl MatchesToken for Vec<Token> {
    #[inline]
    fn matches(&self, token: &Token) -> bool {
        self.contains(token)
    }
}

impl<const T: usize> MatchesToken for [Token; T] {
    #[inline]
    fn matches(&self, token: &Token) -> bool {
        self.contains(token)
    }
}
impl<const T: usize> MatchesToken for [TokenType; T] {
    #[inline]
    fn matches(&self, token: &Token) -> bool {
        self.contains(&token.token_type)
    }
}

impl<T: MatchesToken> ParseWithArgs<T> for Block {
    fn parse_with(
        mut token: Token,
        lexer: &mut Lexer,
        errors: &mut Vec<ParseError>,
        stop_at: T,
    ) -> Option<Self> {
        let mut statements = Vec::new();
        let mut last_statement = None;

        if stop_at.matches(&token) {
            return (!statements.is_empty() || last_statement.is_some()).then_some(Self {
                statements,
                last_statement,
            });
        }

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
                statements.push((Pointer::new(statement), maybe_semicolon))
            } else if let Some(statement) =
                TerminationStatement::parse(token.clone(), lexer, errors)
            {
                maybe_next_token!(lexer, maybe_semicolon, TokenType::Symbol(Symbol::Semicolon));
                last_statement = Some((Pointer::new(statement), maybe_semicolon));
            } else {
                failed_parsing = true;
            }

            let state = lexer.save_state();
            let next_token = lexer.next_token();

            if stop_at.matches(&next_token) {
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

        (!statements.is_empty() || last_statement.is_some()).then_some(Self {
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

fn get_range<T: GetRange>(
    statement: &T,
    semi_colon: &Option<Token>,
) -> Result<Range, GetRangeError> {
    let statement_range = statement.get_range();

    if let Some(seme_colon) = semi_colon {
        Ok(Range::new(
            statement_range?.start,
            seme_colon.get_range()?.end,
        ))
    } else {
        statement_range
    }
}

impl GetRange for Block {
    fn get_range(&self) -> Result<Range, GetRangeError> {
        if self.is_empty() {
            return Err(GetRangeError::EmptyBlock);
        }
        if let Some((first_statement, semi_colon)) = self.statements.first() {
            let last_statement_range = match &self.last_statement {
                Some((statement, semi_colon)) => get_range(statement, semi_colon),
                None => self
                    .statements
                    .first()
                    .map(|(statement, semi_colon)| get_range(statement, semi_colon))
                    .unwrap(), // We're sure that at least one statement exists.
            };

            return Ok(Range::new(
                get_range(first_statement, semi_colon)?.start,
                last_statement_range?.end,
            ));
        }

        match &self.last_statement {
            Some((statement, semi_colon)) => get_range(statement, semi_colon),
            None => Err(GetRangeError::EmptyBlock),
            // `None` should be `unreachable!()`.
        }
    }
}

impl Print for Block {
    fn print(&self) -> String {
        if self.is_empty() {
            String::new()
        } else if self.statements.is_empty() {
            self.last_statement.as_ref().unwrap().print()
        } else if self.last_statement.is_none() {
            self.statements.print()
        } else {
            self.statements.print().trim_end().to_string()
                + &self.last_statement.as_ref().unwrap().print()
        }
    }
}
