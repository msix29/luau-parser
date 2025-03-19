use luau_lexer::prelude::{Lexer, ParseError, Token, TokenType};

use crate::types::{Block, ParseWithArgs};

impl ParseWithArgs<Option<TokenType>> for Block {
    fn parse_with(
        token: Token,
        lexer: &mut Lexer,
        errors: &mut Vec<ParseError>,
        stop_at: Option<TokenType>,
    ) -> Option<Self> {
        todo!()
    }
}
